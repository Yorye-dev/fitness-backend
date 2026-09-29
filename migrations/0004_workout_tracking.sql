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
