-- Optional entry-specific portions. Historical rows remain unchanged in grams.
ALTER TABLE meal_log_items
    ADD COLUMN portion_count NUMERIC(10,3),
    ADD COLUMN portion_grams NUMERIC(10,3),
    ADD CONSTRAINT meal_log_items_portions_check CHECK (
        (portion_count IS NULL AND portion_grams IS NULL) OR
        (portion_count IS NOT NULL AND portion_grams IS NOT NULL
         AND portion_count BETWEEN 0.001 AND 1000000
         AND portion_grams BETWEEN 0.001 AND 1000000
         AND quantity_grams = round(portion_count * portion_grams, 3))
    );

ALTER TABLE legacy.daily_consumption
    ADD COLUMN portion_count NUMERIC(10,3),
    ADD COLUMN portion_grams NUMERIC(10,3),
    ADD CONSTRAINT daily_consumption_portions_check CHECK (
        (portion_count IS NULL AND portion_grams IS NULL) OR
        (portion_count IS NOT NULL AND portion_grams IS NOT NULL
         AND portion_count BETWEEN 0.001 AND 1000000
         AND portion_grams BETWEEN 0.001 AND 1000000
         AND quantity_grams = round(portion_count * portion_grams, 3)::REAL)
    );

-- Append columns without changing the view's existing types or dependants.
CREATE OR REPLACE VIEW consumption_entries AS
SELECT i.id, i.user_id, l.local_date AS date, i.food_id AS meal_id, i.quantity_grams,
       i.calories_consumed, i.protein_consumed, i.carbs_consumed, i.fat_consumed,
       i.food_name_snapshot AS meal_name,
       i.calories_per_100g_snapshot AS calories_per_100g,
       i.protein_per_100g_snapshot AS protein_per_100g,
       i.carbs_per_100g_snapshot AS carbs_per_100g,
       i.fat_per_100g_snapshot AS fat_per_100g, l.id AS meal_log_id,
       i.portion_count, i.portion_grams
FROM meal_log_items i JOIN meal_logs l ON l.user_id=i.user_id AND l.id=i.meal_log_id
UNION ALL
SELECT d.id, d.user_id, d.date, d.meal_id, d.quantity_grams::TEXT::NUMERIC,
       d.calories_consumed::TEXT::NUMERIC, d.protein_consumed::TEXT::NUMERIC,
       d.carbs_consumed::TEXT::NUMERIC, d.fat_consumed::TEXT::NUMERIC, m.name,
       m.calories_per_100g::TEXT::NUMERIC, m.protein_per_100g::TEXT::NUMERIC,
       m.carbs_per_100g::TEXT::NUMERIC, m.fat_per_100g::TEXT::NUMERIC, d.id,
       d.portion_count, d.portion_grams
FROM legacy.daily_consumption d JOIN legacy.meals m ON m.id=d.meal_id AND m.user_id=d.user_id;
