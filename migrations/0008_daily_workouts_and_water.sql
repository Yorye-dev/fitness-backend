-- Daily sessions keep immutable exercise identities and prescription snapshots.
ALTER TABLE workout_sessions
    ADD COLUMN is_daily BOOLEAN NOT NULL DEFAULT false,
    ADD COLUMN time_is_estimated BOOLEAN NOT NULL DEFAULT false,
    ADD COLUMN revision INTEGER NOT NULL DEFAULT 0 CHECK (revision >= 0),
    ADD COLUMN last_write_id UUID;
CREATE UNIQUE INDEX workout_sessions_daily_slot_idx ON workout_sessions(user_id, local_date)
    WHERE is_daily;
ALTER TABLE session_exercises ADD COLUMN status TEXT NOT NULL DEFAULT 'pending'
    CHECK (status IN ('pending', 'completed', 'skipped'));

-- Versioned goals retain earlier targets when the user changes today's goal.
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
CREATE INDEX water_intakes_daily_idx ON water_intakes(user_id, local_date, created_at, id)
    WHERE deleted_at IS NULL;
CREATE TRIGGER water_goal_versions_updated_at BEFORE UPDATE ON water_goal_versions
    FOR EACH ROW EXECUTE FUNCTION touch_updated_at();
