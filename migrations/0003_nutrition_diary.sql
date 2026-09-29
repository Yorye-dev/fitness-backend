-- Stop instead of hiding invalid cross-user links in the legacy read view.
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM daily_consumption d JOIN meals m ON m.id=d.meal_id WHERE m.user_id<>d.user_id
    ) THEN
        RAISE EXCEPTION 'Existing consumption references another user''s food; correct ownership before migrating';
    END IF;
END;
$$;

-- Vigente desde effective_from hasta la siguiente version del mismo usuario.
-- La aplicacion inserta nuevas versiones; no sobrescribe el historico.
CREATE TABLE nutrition_goal_versions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    effective_from DATE NOT NULL CHECK (isfinite(effective_from)),
    calories_target NUMERIC(10,3) NOT NULL
        CHECK (calories_target > 0 AND calories_target < 'Infinity'::NUMERIC),
    protein_target NUMERIC(10,3) NOT NULL
        CHECK (protein_target >= 0 AND protein_target < 'Infinity'::NUMERIC),
    carbs_target NUMERIC(10,3) NOT NULL
        CHECK (carbs_target >= 0 AND carbs_target < 'Infinity'::NUMERIC),
    fat_target NUMERIC(10,3) NOT NULL
        CHECK (fat_target >= 0 AND fat_target < 'Infinity'::NUMERIC),
    bmr_estimate NUMERIC(10,3)
        CHECK (bmr_estimate > 0 AND bmr_estimate < 'Infinity'::NUMERIC),
    tdee_estimate NUMERIC(10,3)
        CHECK (tdee_estimate > 0 AND tdee_estimate < 'Infinity'::NUMERIC),
    source TEXT NOT NULL CHECK (source IN ('profile', 'manual', 'migration')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (user_id, effective_from)
);

-- Una cabecera por comida: pueden existir varios snacks o desayunos el mismo dia.
CREATE TABLE meal_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    local_date DATE NOT NULL CHECK (isfinite(local_date)),
    consumed_at TIMESTAMPTZ NOT NULL CHECK (isfinite(consumed_at)),
    time_zone TEXT NOT NULL CHECK (length(btrim(time_zone)) > 0),
    time_is_estimated BOOLEAN NOT NULL DEFAULT false,
    meal_type TEXT NOT NULL CHECK (meal_type IN ('breakfast', 'lunch', 'dinner', 'snack', 'other')),
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (user_id, id),
    CHECK (local_date = (consumed_at AT TIME ZONE time_zone)::DATE)
);
CREATE INDEX meal_logs_user_date_idx ON meal_logs(user_id, local_date, consumed_at, id);

-- Copia de nombre y nutrientes: editar foods no reescribe las ingestas anteriores.
-- Los totales generados usan solo datos de esta fila y no se pueden enviar como valores libres.
CREATE TABLE meal_log_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    meal_log_id UUID NOT NULL,
    food_id UUID NOT NULL,
    position INTEGER NOT NULL CHECK (position > 0),
    quantity_grams NUMERIC(10,3) NOT NULL
        CHECK (quantity_grams > 0 AND quantity_grams < 'Infinity'::NUMERIC),
    food_name_snapshot TEXT NOT NULL CHECK (char_length(btrim(food_name_snapshot)) BETWEEN 1 AND 200),
    calories_per_100g_snapshot NUMERIC(8,3) NOT NULL
        CHECK (calories_per_100g_snapshot >= 0 AND calories_per_100g_snapshot < 'Infinity'::NUMERIC),
    protein_per_100g_snapshot NUMERIC(8,3) NOT NULL CHECK (protein_per_100g_snapshot BETWEEN 0 AND 100),
    carbs_per_100g_snapshot NUMERIC(8,3) NOT NULL CHECK (carbs_per_100g_snapshot BETWEEN 0 AND 100),
    fat_per_100g_snapshot NUMERIC(8,3) NOT NULL CHECK (fat_per_100g_snapshot BETWEEN 0 AND 100),
    calories_consumed NUMERIC(20,8) GENERATED ALWAYS AS
        (quantity_grams * calories_per_100g_snapshot * 0.01) STORED,
    protein_consumed NUMERIC(20,8) GENERATED ALWAYS AS
        (quantity_grams * protein_per_100g_snapshot * 0.01) STORED,
    carbs_consumed NUMERIC(20,8) GENERATED ALWAYS AS
        (quantity_grams * carbs_per_100g_snapshot * 0.01) STORED,
    fat_consumed NUMERIC(20,8) GENERATED ALWAYS AS
        (quantity_grams * fat_per_100g_snapshot * 0.01) STORED,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (user_id, meal_log_id, position),
    FOREIGN KEY (user_id, meal_log_id) REFERENCES meal_logs(user_id, id) ON DELETE CASCADE,
    FOREIGN KEY (user_id, food_id) REFERENCES foods(user_id, id) ON DELETE NO ACTION
);
CREATE INDEX meal_log_items_food_idx ON meal_log_items(user_id, food_id);

-- Existing goals have no historical date. Import them as a baseline for known consumption
-- dates, or today when no consumption exists. Their source explicitly records this limitation.
INSERT INTO nutrition_goal_versions
    (id, user_id, effective_from, calories_target, protein_target, carbs_target, fat_target,
     bmr_estimate, tdee_estimate, source)
SELECT g.id, g.user_id,
       LEAST(CURRENT_DATE, COALESCE((SELECT MIN(d.date) FROM daily_consumption d WHERE d.user_id=g.user_id), CURRENT_DATE)),
       g.tdee::TEXT::NUMERIC, g.protein_goal::TEXT::NUMERIC, g.carbs_goal::TEXT::NUMERIC,
       g.fats_goal::TEXT::NUMERIC,
       CASE WHEN g.bmr > 0 AND g.bmr < 'Infinity'::REAL THEN g.bmr::TEXT::NUMERIC END,
       CASE WHEN g.tdee > 0 AND g.tdee < 'Infinity'::REAL THEN g.tdee::TEXT::NUMERIC END,
       'migration'
FROM users_nutrition_goals g;

-- Preserve the original consumption totals: original times and nutrient snapshots cannot
-- be reconstructed reliably. New writes use only the new public tables.
CREATE SCHEMA legacy;
ALTER TABLE daily_consumption SET SCHEMA legacy;
ALTER TABLE meals SET SCHEMA legacy;
ALTER TABLE users_nutrition_goals SET SCHEMA legacy;

CREATE VIEW consumption_entries AS
SELECT i.id, i.user_id, l.local_date AS date, i.food_id AS meal_id, i.quantity_grams,
       i.calories_consumed, i.protein_consumed, i.carbs_consumed, i.fat_consumed,
       i.food_name_snapshot AS meal_name,
       i.calories_per_100g_snapshot AS calories_per_100g,
       i.protein_per_100g_snapshot AS protein_per_100g,
       i.carbs_per_100g_snapshot AS carbs_per_100g,
       i.fat_per_100g_snapshot AS fat_per_100g, l.id AS meal_log_id
FROM meal_log_items i JOIN meal_logs l ON l.user_id=i.user_id AND l.id=i.meal_log_id
UNION ALL
SELECT d.id, d.user_id, d.date, d.meal_id, d.quantity_grams::TEXT::NUMERIC,
       d.calories_consumed::TEXT::NUMERIC, d.protein_consumed::TEXT::NUMERIC,
       d.carbs_consumed::TEXT::NUMERIC, d.fat_consumed::TEXT::NUMERIC, m.name,
       m.calories_per_100g::TEXT::NUMERIC, m.protein_per_100g::TEXT::NUMERIC,
       m.carbs_per_100g::TEXT::NUMERIC, m.fat_per_100g::TEXT::NUMERIC, d.id
FROM legacy.daily_consumption d JOIN legacy.meals m ON m.id=d.meal_id AND m.user_id=d.user_id;

CREATE VIEW daily_nutrition_totals AS
SELECT user_id, date AS local_date, COUNT(DISTINCT meal_log_id) AS meal_count, COUNT(*) AS item_count,
       SUM(calories_consumed) AS calories_consumed, SUM(protein_consumed) AS protein_consumed,
       SUM(carbs_consumed) AS carbs_consumed, SUM(fat_consumed) AS fat_consumed
FROM consumption_entries GROUP BY user_id, date;

CREATE TRIGGER meal_logs_updated_at BEFORE UPDATE ON meal_logs
    FOR EACH ROW EXECUTE FUNCTION touch_updated_at();
CREATE TRIGGER meal_log_items_updated_at BEFORE UPDATE ON meal_log_items
    FOR EACH ROW EXECUTE FUNCTION touch_updated_at();
