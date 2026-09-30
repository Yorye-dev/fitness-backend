-- Current recurring plan. Weekdays use ISO numbering (Monday=1, Sunday=7).
-- Execution history is kept separately in workout_sessions and its snapshots.
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
