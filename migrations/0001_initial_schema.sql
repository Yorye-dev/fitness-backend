CREATE TYPE sex AS ENUM ('male', 'female');
CREATE TYPE activity_level AS ENUM (
    'sedentary',
    'lightly_active',
    'moderately_active',
    'very_active',
    'extra_active'
);
CREATE TYPE goal AS ENUM ('lose_weight', 'maintain', 'gain_muscle');

CREATE TABLE users (
    id UUID PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    age INTEGER NOT NULL,
    sex sex NOT NULL,
    height INTEGER NOT NULL,
    weight REAL NOT NULL,
    activity_level activity_level NOT NULL,
    goal goal NOT NULL
);

CREATE TABLE users_nutrition_goals (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL UNIQUE REFERENCES users (id) ON DELETE CASCADE,
    protein_goal REAL NOT NULL,
    carbs_goal REAL NOT NULL,
    fats_goal REAL NOT NULL,
    tdee REAL NOT NULL,
    bmr REAL NOT NULL
);

CREATE TABLE meals (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    calories_per_100g REAL NOT NULL,
    protein_per_100g REAL NOT NULL,
    carbs_per_100g REAL NOT NULL,
    fat_per_100g REAL NOT NULL
);

CREATE INDEX meals_user_id_name_idx ON meals (user_id, name);

CREATE TABLE daily_consumption (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    date DATE NOT NULL,
    meal_id UUID NOT NULL REFERENCES meals (id),
    quantity_grams REAL NOT NULL,
    calories_consumed REAL NOT NULL,
    protein_consumed REAL NOT NULL,
    carbs_consumed REAL NOT NULL,
    fat_consumed REAL NOT NULL
);

CREATE INDEX daily_consumption_user_date_idx ON daily_consumption (user_id, date DESC);
