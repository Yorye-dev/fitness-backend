-- Extend the existing schema. 0001 remains immutable.
-- SQLx executes this migration in a transaction: invalid existing data aborts it.
CREATE FUNCTION touch_updated_at() RETURNS TRIGGER LANGUAGE plpgsql AS $$
BEGIN
    NEW.updated_at := CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$;

ALTER TABLE users
    ALTER COLUMN id SET DEFAULT gen_random_uuid(),
    ALTER COLUMN weight TYPE NUMERIC(8,3) USING weight::TEXT::NUMERIC,
    ADD COLUMN time_zone TEXT NOT NULL DEFAULT 'UTC',
    ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    ADD CONSTRAINT users_username_length CHECK (char_length(username) BETWEEN 3 AND 50),
    ADD CONSTRAINT users_password_hash_present CHECK (length(password_hash) > 0),
    ADD CONSTRAINT users_weight_range CHECK (weight > 0 AND weight <= 300),
    ADD CONSTRAINT users_height_range CHECK (height BETWEEN 1 AND 250),
    ADD CONSTRAINT users_age_range CHECK (age BETWEEN 1 AND 120),
    ADD CONSTRAINT users_time_zone_present CHECK (length(btrim(time_zone)) > 0);

-- Catalogo privado. El alimento se archiva para conservar sus referencias historicas.
CREATE TABLE foods (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL CHECK (char_length(btrim(name)) BETWEEN 1 AND 200),
    brand TEXT,
    calories_per_100g NUMERIC(8,3) NOT NULL
        CHECK (calories_per_100g >= 0 AND calories_per_100g < 'Infinity'::NUMERIC),
    protein_per_100g NUMERIC(8,3) NOT NULL CHECK (protein_per_100g BETWEEN 0 AND 100),
    carbs_per_100g NUMERIC(8,3) NOT NULL CHECK (carbs_per_100g BETWEEN 0 AND 100),
    fat_per_100g NUMERIC(8,3) NOT NULL CHECK (fat_per_100g BETWEEN 0 AND 100),
    archived_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (user_id, id)
);
CREATE INDEX foods_active_name_idx ON foods(user_id, name, id) WHERE archived_at IS NULL;

INSERT INTO foods (id, user_id, name, calories_per_100g, protein_per_100g, carbs_per_100g, fat_per_100g)
SELECT id, user_id, name, calories_per_100g::TEXT::NUMERIC, protein_per_100g::TEXT::NUMERIC,
       carbs_per_100g::TEXT::NUMERIC, fat_per_100g::TEXT::NUMERIC
FROM meals;

CREATE TRIGGER users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION touch_updated_at();
CREATE TRIGGER foods_updated_at BEFORE UPDATE ON foods
    FOR EACH ROW EXECUTE FUNCTION touch_updated_at();
