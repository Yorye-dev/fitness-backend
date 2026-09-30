-- Foods can be measured by weight or by unit, without inventing a unit weight.
-- Existing foods and historical snapshots retain the per-100g basis.
DROP VIEW daily_nutrition_totals;
DROP VIEW consumption_entries;

ALTER TABLE foods
    ADD COLUMN nutrition_basis TEXT NOT NULL DEFAULT 'per_100g' CHECK (nutrition_basis IN ('per_100g', 'per_unit')),
    ALTER COLUMN calories_per_100g DROP NOT NULL,
    ALTER COLUMN protein_per_100g DROP NOT NULL,
    ALTER COLUMN carbs_per_100g DROP NOT NULL,
    ALTER COLUMN fat_per_100g DROP NOT NULL,
    ADD COLUMN calories_per_unit NUMERIC(8,3) CHECK (calories_per_unit >= 0 AND calories_per_unit < 'Infinity'::NUMERIC),
    ADD COLUMN protein_per_unit NUMERIC(8,3) CHECK (protein_per_unit >= 0 AND protein_per_unit < 'Infinity'::NUMERIC),
    ADD COLUMN carbs_per_unit NUMERIC(8,3) CHECK (carbs_per_unit >= 0 AND carbs_per_unit < 'Infinity'::NUMERIC),
    ADD COLUMN fat_per_unit NUMERIC(8,3) CHECK (fat_per_unit >= 0 AND fat_per_unit < 'Infinity'::NUMERIC),
    ADD CONSTRAINT foods_nutrition_values_check CHECK (
        (nutrition_basis='per_100g' AND calories_per_100g IS NOT NULL AND protein_per_100g IS NOT NULL AND carbs_per_100g IS NOT NULL AND fat_per_100g IS NOT NULL AND calories_per_unit IS NULL AND protein_per_unit IS NULL AND carbs_per_unit IS NULL AND fat_per_unit IS NULL)
        OR (nutrition_basis='per_unit' AND calories_per_unit IS NOT NULL AND protein_per_unit IS NOT NULL AND carbs_per_unit IS NOT NULL AND fat_per_unit IS NOT NULL AND calories_per_100g IS NULL AND protein_per_100g IS NULL AND carbs_per_100g IS NULL AND fat_per_100g IS NULL)
    );

ALTER TABLE meal_log_items
    ADD COLUMN nutrition_basis TEXT NOT NULL DEFAULT 'per_100g' CHECK (nutrition_basis IN ('per_100g', 'per_unit')),
    ALTER COLUMN quantity_grams DROP NOT NULL,
    ALTER COLUMN calories_per_100g_snapshot DROP NOT NULL,
    ALTER COLUMN protein_per_100g_snapshot DROP NOT NULL,
    ALTER COLUMN carbs_per_100g_snapshot DROP NOT NULL,
    ALTER COLUMN fat_per_100g_snapshot DROP NOT NULL,
    ADD COLUMN calories_per_unit_snapshot NUMERIC(8,3) CHECK (calories_per_unit_snapshot >= 0 AND calories_per_unit_snapshot < 'Infinity'::NUMERIC),
    ADD COLUMN protein_per_unit_snapshot NUMERIC(8,3) CHECK (protein_per_unit_snapshot >= 0 AND protein_per_unit_snapshot < 'Infinity'::NUMERIC),
    ADD COLUMN carbs_per_unit_snapshot NUMERIC(8,3) CHECK (carbs_per_unit_snapshot >= 0 AND carbs_per_unit_snapshot < 'Infinity'::NUMERIC),
    ADD COLUMN fat_per_unit_snapshot NUMERIC(8,3) CHECK (fat_per_unit_snapshot >= 0 AND fat_per_unit_snapshot < 'Infinity'::NUMERIC),
    DROP CONSTRAINT meal_log_items_portions_check,
    ADD CONSTRAINT meal_log_items_nutrition_values_check CHECK (
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
    DROP COLUMN calories_consumed,
    DROP COLUMN protein_consumed,
    DROP COLUMN carbs_consumed,
    DROP COLUMN fat_consumed;

-- Recreate the generated totals with the same names and precision.
ALTER TABLE meal_log_items
    ADD COLUMN calories_consumed NUMERIC(20,8) GENERATED ALWAYS AS
        (CASE WHEN nutrition_basis='per_unit' THEN portion_count * calories_per_unit_snapshot
         ELSE quantity_grams * calories_per_100g_snapshot * 0.01 END) STORED,
    ADD COLUMN protein_consumed NUMERIC(20,8) GENERATED ALWAYS AS
        (CASE WHEN nutrition_basis='per_unit' THEN portion_count * protein_per_unit_snapshot
         ELSE quantity_grams * protein_per_100g_snapshot * 0.01 END) STORED,
    ADD COLUMN carbs_consumed NUMERIC(20,8) GENERATED ALWAYS AS
        (CASE WHEN nutrition_basis='per_unit' THEN portion_count * carbs_per_unit_snapshot
         ELSE quantity_grams * carbs_per_100g_snapshot * 0.01 END) STORED,
    ADD COLUMN fat_consumed NUMERIC(20,8) GENERATED ALWAYS AS
        (CASE WHEN nutrition_basis='per_unit' THEN portion_count * fat_per_unit_snapshot
         ELSE quantity_grams * fat_per_100g_snapshot * 0.01 END) STORED;

CREATE VIEW consumption_entries AS
SELECT i.id, i.user_id, l.local_date AS date, i.food_id AS meal_id, i.quantity_grams,
       i.calories_consumed, i.protein_consumed, i.carbs_consumed, i.fat_consumed,
       i.food_name_snapshot AS meal_name,
       i.calories_per_100g_snapshot AS calories_per_100g,
       i.protein_per_100g_snapshot AS protein_per_100g,
       i.carbs_per_100g_snapshot AS carbs_per_100g,
       i.fat_per_100g_snapshot AS fat_per_100g,
       l.id AS meal_log_id, i.portion_count, i.portion_grams, i.nutrition_basis,
       i.calories_per_unit_snapshot AS calories_per_unit,
       i.protein_per_unit_snapshot AS protein_per_unit,
       i.carbs_per_unit_snapshot AS carbs_per_unit,
       i.fat_per_unit_snapshot AS fat_per_unit
FROM meal_log_items i JOIN meal_logs l ON l.user_id=i.user_id AND l.id=i.meal_log_id
UNION ALL
SELECT d.id, d.user_id, d.date, d.meal_id, d.quantity_grams::TEXT::NUMERIC,
       d.calories_consumed::TEXT::NUMERIC, d.protein_consumed::TEXT::NUMERIC,
       d.carbs_consumed::TEXT::NUMERIC, d.fat_consumed::TEXT::NUMERIC, m.name,
       m.calories_per_100g::TEXT::NUMERIC, m.protein_per_100g::TEXT::NUMERIC,
       m.carbs_per_100g::TEXT::NUMERIC, m.fat_per_100g::TEXT::NUMERIC, d.id,
       d.portion_count, d.portion_grams, 'per_100g'::TEXT,
       NULL::NUMERIC, NULL::NUMERIC, NULL::NUMERIC, NULL::NUMERIC
FROM legacy.daily_consumption d JOIN legacy.meals m ON m.id=d.meal_id AND m.user_id=d.user_id;

CREATE VIEW daily_nutrition_totals AS
SELECT user_id, date AS local_date, COUNT(DISTINCT meal_log_id) AS meal_count, COUNT(*) AS item_count,
       SUM(calories_consumed) AS calories_consumed, SUM(protein_consumed) AS protein_consumed,
       SUM(carbs_consumed) AS carbs_consumed, SUM(fat_consumed) AS fat_consumed
FROM consumption_entries GROUP BY user_id, date;
