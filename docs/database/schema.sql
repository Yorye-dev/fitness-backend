-- Fitness: modelo objetivo para PostgreSQL 17.
-- DDL de referencia del modelo activo para una BASE VACIA.
-- El backend se instala/actualiza con migrations/0001..0008, no ejecutando este archivo.
-- La migracion 0003 conserva ademas las tablas anteriores en legacy y las integra en las vistas.
-- Sin datos de ejemplo, extensiones ni instrucciones de borrado.
BEGIN;

CREATE TYPE sex AS ENUM ('male', 'female');
CREATE TYPE activity_level AS ENUM (
    'sedentary', 'lightly_active', 'moderately_active', 'very_active', 'extra_active'
);
CREATE TYPE goal AS ENUM ('lose_weight', 'maintain', 'gain_muscle');

-- NUMERIC(p,s) limita la magnitud; las comparaciones con Infinity excluyen tambien NaN.
-- Un CHECK sobre una columna opcional permite NULL; NOT NULL se declara por separado.
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username TEXT NOT NULL UNIQUE CHECK (char_length(username) BETWEEN 3 AND 50),
    password_hash TEXT NOT NULL CHECK (length(password_hash) > 0),
    sex sex NOT NULL,
    weight NUMERIC(8,3) NOT NULL CHECK (weight > 0 AND weight <= 300),
    height INTEGER NOT NULL CHECK (height BETWEEN 1 AND 250),
    age INTEGER NOT NULL CHECK (age BETWEEN 1 AND 120),
    activity_level activity_level NOT NULL,
    goal goal NOT NULL,
    time_zone TEXT NOT NULL DEFAULT 'UTC' CHECK (length(btrim(time_zone)) > 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- NUTRICION ---------------------------------------------------------------
-- Catalogo privado. El alimento se archiva para conservar sus referencias historicas.
CREATE TABLE foods (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL CHECK (char_length(btrim(name)) BETWEEN 1 AND 200),
    brand TEXT,
    nutrition_basis TEXT NOT NULL DEFAULT 'per_100g' CHECK (nutrition_basis IN ('per_100g', 'per_unit')),
    calories_per_100g NUMERIC(8,3)
        CHECK (calories_per_100g >= 0 AND calories_per_100g < 'Infinity'::NUMERIC),
    protein_per_100g NUMERIC(8,3) CHECK (protein_per_100g BETWEEN 0 AND 100),
    carbs_per_100g NUMERIC(8,3) CHECK (carbs_per_100g BETWEEN 0 AND 100),
    fat_per_100g NUMERIC(8,3) CHECK (fat_per_100g BETWEEN 0 AND 100),
    calories_per_unit NUMERIC(8,3) CHECK (calories_per_unit >= 0 AND calories_per_unit < 'Infinity'::NUMERIC),
    protein_per_unit NUMERIC(8,3) CHECK (protein_per_unit >= 0 AND protein_per_unit < 'Infinity'::NUMERIC),
    carbs_per_unit NUMERIC(8,3) CHECK (carbs_per_unit >= 0 AND carbs_per_unit < 'Infinity'::NUMERIC),
    fat_per_unit NUMERIC(8,3) CHECK (fat_per_unit >= 0 AND fat_per_unit < 'Infinity'::NUMERIC),
    CONSTRAINT foods_nutrition_values_check CHECK (
        (nutrition_basis='per_100g' AND calories_per_100g IS NOT NULL AND protein_per_100g IS NOT NULL AND carbs_per_100g IS NOT NULL AND fat_per_100g IS NOT NULL AND calories_per_unit IS NULL AND protein_per_unit IS NULL AND carbs_per_unit IS NULL AND fat_per_unit IS NULL)
        OR (nutrition_basis='per_unit' AND calories_per_unit IS NOT NULL AND protein_per_unit IS NOT NULL AND carbs_per_unit IS NOT NULL AND fat_per_unit IS NOT NULL AND calories_per_100g IS NULL AND protein_per_100g IS NULL AND carbs_per_100g IS NULL AND fat_per_100g IS NULL)
    ),
    archived_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (user_id, id)
);
CREATE INDEX foods_active_name_idx ON foods(user_id, name, id) WHERE archived_at IS NULL;

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
    nutrition_basis TEXT NOT NULL DEFAULT 'per_100g' CHECK (nutrition_basis IN ('per_100g', 'per_unit')),
    quantity_grams NUMERIC(10,3)
        CHECK (quantity_grams > 0 AND quantity_grams < 'Infinity'::NUMERIC),
    portion_count NUMERIC(10,3),
    portion_grams NUMERIC(10,3),
    food_name_snapshot TEXT NOT NULL CHECK (char_length(btrim(food_name_snapshot)) BETWEEN 1 AND 200),
    calories_per_100g_snapshot NUMERIC(8,3)
        CHECK (calories_per_100g_snapshot >= 0 AND calories_per_100g_snapshot < 'Infinity'::NUMERIC),
    protein_per_100g_snapshot NUMERIC(8,3) CHECK (protein_per_100g_snapshot BETWEEN 0 AND 100),
    carbs_per_100g_snapshot NUMERIC(8,3) CHECK (carbs_per_100g_snapshot BETWEEN 0 AND 100),
    fat_per_100g_snapshot NUMERIC(8,3) CHECK (fat_per_100g_snapshot BETWEEN 0 AND 100),
    calories_per_unit_snapshot NUMERIC(8,3) CHECK (calories_per_unit_snapshot >= 0 AND calories_per_unit_snapshot < 'Infinity'::NUMERIC),
    protein_per_unit_snapshot NUMERIC(8,3) CHECK (protein_per_unit_snapshot >= 0 AND protein_per_unit_snapshot < 'Infinity'::NUMERIC),
    carbs_per_unit_snapshot NUMERIC(8,3) CHECK (carbs_per_unit_snapshot >= 0 AND carbs_per_unit_snapshot < 'Infinity'::NUMERIC),
    fat_per_unit_snapshot NUMERIC(8,3) CHECK (fat_per_unit_snapshot >= 0 AND fat_per_unit_snapshot < 'Infinity'::NUMERIC),
    CONSTRAINT meal_log_items_nutrition_values_check CHECK (
        (nutrition_basis='per_100g' AND quantity_grams IS NOT NULL
         AND calories_per_100g_snapshot IS NOT NULL AND protein_per_100g_snapshot IS NOT NULL AND carbs_per_100g_snapshot IS NOT NULL AND fat_per_100g_snapshot IS NOT NULL AND calories_per_unit_snapshot IS NULL AND protein_per_unit_snapshot IS NULL AND carbs_per_unit_snapshot IS NULL AND fat_per_unit_snapshot IS NULL
         AND ((portion_count IS NULL AND portion_grams IS NULL)
           OR (portion_count IS NOT NULL AND portion_grams IS NOT NULL
             AND portion_count BETWEEN 0.001 AND 1000000 AND portion_grams BETWEEN 0.001 AND 1000000
             AND quantity_grams = round(portion_count * portion_grams, 3))))
        OR (nutrition_basis='per_unit' AND quantity_grams IS NULL AND portion_grams IS NULL
         AND portion_count IS NOT NULL AND portion_count BETWEEN 0.001 AND 1000000
         AND calories_per_unit_snapshot IS NOT NULL AND protein_per_unit_snapshot IS NOT NULL AND carbs_per_unit_snapshot IS NOT NULL AND fat_per_unit_snapshot IS NOT NULL AND calories_per_100g_snapshot IS NULL AND protein_per_100g_snapshot IS NULL AND carbs_per_100g_snapshot IS NULL AND fat_per_100g_snapshot IS NULL)
    ),
    calories_consumed NUMERIC(20,8) GENERATED ALWAYS AS
        (CASE WHEN nutrition_basis='per_unit' THEN portion_count * calories_per_unit_snapshot
         ELSE quantity_grams * calories_per_100g_snapshot * 0.01 END) STORED,
    protein_consumed NUMERIC(20,8) GENERATED ALWAYS AS
        (CASE WHEN nutrition_basis='per_unit' THEN portion_count * protein_per_unit_snapshot
         ELSE quantity_grams * protein_per_100g_snapshot * 0.01 END) STORED,
    carbs_consumed NUMERIC(20,8) GENERATED ALWAYS AS
        (CASE WHEN nutrition_basis='per_unit' THEN portion_count * carbs_per_unit_snapshot
         ELSE quantity_grams * carbs_per_100g_snapshot * 0.01 END) STORED,
    fat_consumed NUMERIC(20,8) GENERATED ALWAYS AS
        (CASE WHEN nutrition_basis='per_unit' THEN portion_count * fat_per_unit_snapshot
         ELSE quantity_grams * fat_per_100g_snapshot * 0.01 END) STORED,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (user_id, meal_log_id, position),
    FOREIGN KEY (user_id, meal_log_id) REFERENCES meal_logs(user_id, id) ON DELETE CASCADE,
    FOREIGN KEY (user_id, food_id) REFERENCES foods(user_id, id) ON DELETE NO ACTION
);
CREATE INDEX meal_log_items_food_idx ON meal_log_items(user_id, food_id);

-- Solo consumos. Los objetivos se seleccionan por fecha en la consulta de resumen.
CREATE VIEW daily_nutrition_totals AS
SELECT
    ml.user_id,
    ml.local_date,
    COUNT(DISTINCT ml.id) AS meal_count,
    COUNT(mi.id) AS item_count,
    COALESCE(SUM(mi.calories_consumed), 0::NUMERIC) AS calories_consumed,
    COALESCE(SUM(mi.protein_consumed), 0::NUMERIC) AS protein_consumed,
    COALESCE(SUM(mi.carbs_consumed), 0::NUMERIC) AS carbs_consumed,
    COALESCE(SUM(mi.fat_consumed), 0::NUMERIC) AS fat_consumed
FROM meal_logs ml
JOIN meal_log_items mi ON mi.user_id = ml.user_id AND mi.meal_log_id = ml.id
GROUP BY ml.user_id, ml.local_date;

-- ENTRENAMIENTO -----------------------------------------------------------
CREATE TABLE exercises (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL CHECK (char_length(btrim(name)) BETWEEN 1 AND 200),
    modality TEXT NOT NULL CHECK (modality IN ('strength', 'cardio', 'mobility')),
    equipment TEXT,
    instructions TEXT,
    archived_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (user_id, id)
);
CREATE INDEX exercises_active_name_idx ON exercises(user_id, name, id) WHERE archived_at IS NULL;

CREATE TABLE workout_routines (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL CHECK (char_length(btrim(name)) BETWEEN 1 AND 200),
    description TEXT,
    archived_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (user_id, id)
);
CREATE INDEX workout_routines_active_name_idx ON workout_routines(user_id, name, id) WHERE archived_at IS NULL;

CREATE TABLE routine_exercises (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    routine_id UUID NOT NULL,
    exercise_id UUID NOT NULL,
    position INTEGER NOT NULL CHECK (position > 0),
    target_sets SMALLINT NOT NULL CHECK (target_sets > 0),
    target_reps_min SMALLINT CHECK (target_reps_min > 0),
    target_reps_max SMALLINT CHECK (target_reps_max > 0),
    target_load_kg NUMERIC(8,3) CHECK (target_load_kg >= 0 AND target_load_kg < 'Infinity'::NUMERIC),
    target_duration_seconds INTEGER CHECK (target_duration_seconds > 0),
    target_distance_m NUMERIC(12,3) CHECK (target_distance_m > 0 AND target_distance_m < 'Infinity'::NUMERIC),
    rest_seconds INTEGER NOT NULL DEFAULT 90 CHECK (rest_seconds >= 0),
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (user_id, routine_id, position),
    FOREIGN KEY (user_id, routine_id) REFERENCES workout_routines(user_id, id) ON DELETE CASCADE,
    FOREIGN KEY (user_id, exercise_id) REFERENCES exercises(user_id, id) ON DELETE NO ACTION,
    CHECK ((target_reps_min IS NULL) = (target_reps_max IS NULL)),
    CHECK (target_reps_max >= target_reps_min),
    CHECK (target_reps_min IS NOT NULL OR target_duration_seconds IS NOT NULL OR target_distance_m IS NOT NULL)
);
CREATE INDEX routine_exercises_exercise_idx ON routine_exercises(user_id, exercise_id);

-- routine_id puede ser NULL para entrenamientos libres. El nombre se copia al empezar.
CREATE TABLE workout_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    routine_id UUID,
    name_snapshot TEXT NOT NULL CHECK (char_length(btrim(name_snapshot)) BETWEEN 1 AND 200),
    local_date DATE NOT NULL CHECK (isfinite(local_date)),
    time_zone TEXT NOT NULL CHECK (length(btrim(time_zone)) > 0),
    started_at TIMESTAMPTZ NOT NULL CHECK (isfinite(started_at)),
    finished_at TIMESTAMPTZ CHECK (isfinite(finished_at)),
    status TEXT NOT NULL DEFAULT 'in_progress' CHECK (status IN ('in_progress', 'completed', 'cancelled')),
    is_daily BOOLEAN NOT NULL DEFAULT false,
    time_is_estimated BOOLEAN NOT NULL DEFAULT false,
    revision INTEGER NOT NULL DEFAULT 0 CHECK (revision >= 0),
    last_write_id UUID,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (user_id, id),
    FOREIGN KEY (user_id, routine_id) REFERENCES workout_routines(user_id, id) ON DELETE NO ACTION,
    CHECK (local_date = (started_at AT TIME ZONE time_zone)::DATE),
    CHECK (finished_at >= started_at),
    CHECK ((status = 'in_progress') = (finished_at IS NULL))
);
CREATE INDEX workout_sessions_user_date_idx ON workout_sessions(user_id, local_date, started_at, id);
CREATE INDEX workout_sessions_routine_idx ON workout_sessions(user_id, routine_id);
CREATE UNIQUE INDEX workout_sessions_daily_slot_idx ON workout_sessions(user_id, local_date) WHERE is_daily;

-- Copia de la prescripcion al iniciar la sesion; no depende de futuras ediciones de la rutina.
-- Los objetivos son opcionales para ejercicios agregados durante un entrenamiento libre.
CREATE TABLE session_exercises (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    session_id UUID NOT NULL,
    exercise_id UUID NOT NULL,
    position INTEGER NOT NULL CHECK (position > 0),
    exercise_name_snapshot TEXT NOT NULL CHECK (char_length(btrim(exercise_name_snapshot)) BETWEEN 1 AND 200),
    modality_snapshot TEXT NOT NULL CHECK (modality_snapshot IN ('strength', 'cardio', 'mobility')),
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'completed', 'skipped')),
    target_sets SMALLINT CHECK (target_sets > 0),
    target_reps_min SMALLINT CHECK (target_reps_min > 0),
    target_reps_max SMALLINT CHECK (target_reps_max > 0),
    target_load_kg NUMERIC(8,3) CHECK (target_load_kg >= 0 AND target_load_kg < 'Infinity'::NUMERIC),
    target_duration_seconds INTEGER CHECK (target_duration_seconds > 0),
    target_distance_m NUMERIC(12,3) CHECK (target_distance_m > 0 AND target_distance_m < 'Infinity'::NUMERIC),
    rest_seconds INTEGER CHECK (rest_seconds >= 0),
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (user_id, id),
    UNIQUE (user_id, session_id, position),
    FOREIGN KEY (user_id, session_id) REFERENCES workout_sessions(user_id, id) ON DELETE CASCADE,
    FOREIGN KEY (user_id, exercise_id) REFERENCES exercises(user_id, id) ON DELETE NO ACTION,
    CHECK ((target_reps_min IS NULL) = (target_reps_max IS NULL)),
    CHECK (target_reps_max >= target_reps_min)
);
CREATE INDEX session_exercises_exercise_idx ON session_exercises(user_id, exercise_id);

-- Metricas reales. La carga es externa: 0 kg significa sin carga anadida.
-- NULL significa que esa metrica no se ha registrado o no aplica.
CREATE TABLE workout_sets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    session_exercise_id UUID NOT NULL,
    set_number SMALLINT NOT NULL CHECK (set_number > 0),
    set_type TEXT NOT NULL DEFAULT 'working' CHECK (set_type IN ('warmup', 'working')),
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'completed', 'skipped')),
    reps SMALLINT CHECK (reps > 0),
    load_kg NUMERIC(8,3) CHECK (load_kg >= 0 AND load_kg < 'Infinity'::NUMERIC),
    duration_seconds INTEGER CHECK (duration_seconds > 0),
    distance_m NUMERIC(12,3) CHECK (distance_m > 0 AND distance_m < 'Infinity'::NUMERIC),
    rpe NUMERIC(3,1) CHECK (rpe BETWEEN 1 AND 10),
    completed_at TIMESTAMPTZ CHECK (isfinite(completed_at)),
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (user_id, session_exercise_id, set_number),
    FOREIGN KEY (user_id, session_exercise_id) REFERENCES session_exercises(user_id, id) ON DELETE CASCADE,
    CHECK ((status = 'completed') = (completed_at IS NOT NULL)),
    CHECK (status <> 'completed' OR reps IS NOT NULL OR duration_seconds IS NOT NULL OR distance_m IS NOT NULL)
);

-- Mantenimiento automatico de updated_at para tablas editables.
CREATE FUNCTION touch_updated_at() RETURNS TRIGGER LANGUAGE plpgsql AS $$
BEGIN
    NEW.updated_at := CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$;

CREATE TRIGGER users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION touch_updated_at();
CREATE TRIGGER foods_updated_at BEFORE UPDATE ON foods
    FOR EACH ROW EXECUTE FUNCTION touch_updated_at();
CREATE TRIGGER meal_logs_updated_at BEFORE UPDATE ON meal_logs
    FOR EACH ROW EXECUTE FUNCTION touch_updated_at();
CREATE TRIGGER meal_log_items_updated_at BEFORE UPDATE ON meal_log_items
    FOR EACH ROW EXECUTE FUNCTION touch_updated_at();
CREATE TRIGGER exercises_updated_at BEFORE UPDATE ON exercises
    FOR EACH ROW EXECUTE FUNCTION touch_updated_at();
CREATE TRIGGER workout_routines_updated_at BEFORE UPDATE ON workout_routines
    FOR EACH ROW EXECUTE FUNCTION touch_updated_at();
CREATE TRIGGER routine_exercises_updated_at BEFORE UPDATE ON routine_exercises
    FOR EACH ROW EXECUTE FUNCTION touch_updated_at();
CREATE TRIGGER workout_sessions_updated_at BEFORE UPDATE ON workout_sessions
    FOR EACH ROW EXECUTE FUNCTION touch_updated_at();
CREATE TRIGGER session_exercises_updated_at BEFORE UPDATE ON session_exercises
    FOR EACH ROW EXECUTE FUNCTION touch_updated_at();
CREATE TRIGGER workout_sets_updated_at BEFORE UPDATE ON workout_sets
    FOR EACH ROW EXECUTE FUNCTION touch_updated_at();

-- Plan semanal recurrente actual; el historial real se conserva en workout_sessions.
CREATE TABLE weekly_workout_schedule (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    weekday SMALLINT NOT NULL CHECK (weekday BETWEEN 1 AND 7),
    routine_id UUID,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, weekday),
    FOREIGN KEY (user_id, routine_id) REFERENCES workout_routines(user_id, id) ON DELETE NO ACTION
);
CREATE INDEX weekly_workout_schedule_routine_idx ON weekly_workout_schedule(user_id, routine_id);
CREATE TRIGGER weekly_workout_schedule_updated_at BEFORE UPDATE ON weekly_workout_schedule
    FOR EACH ROW EXECUTE FUNCTION touch_updated_at();

CREATE TABLE water_goal_versions (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    effective_from DATE NOT NULL CHECK (isfinite(effective_from)),
    goal_ml INTEGER NOT NULL CHECK (goal_ml BETWEEN 100 AND 10000),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, effective_from)
);
CREATE TABLE water_intakes (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    local_date DATE NOT NULL CHECK (isfinite(local_date)),
    amount_ml INTEGER NOT NULL CHECK (amount_ml BETWEEN 1 AND 5000),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ
);
CREATE INDEX water_intakes_daily_idx ON water_intakes(user_id, local_date, created_at, id) WHERE deleted_at IS NULL;
CREATE TRIGGER water_goal_versions_updated_at BEFORE UPDATE ON water_goal_versions
    FOR EACH ROW EXECUTE FUNCTION touch_updated_at();

COMMIT;
