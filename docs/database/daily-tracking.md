# Registro diario: sesiones y agua

## Entrenamientos

`POST /api/training/sessions` recibe `date` (`YYYY-MM-DD`) y `routine_id`. Obtiene el usuario del
access token, comprueba la propiedad y copia la rutina y sus ejercicios en una transacción.
Un índice parcial garantiza una sesión de la home por `(user_id, local_date)` con `is_daily = true`.
Reintentar el inicio devuelve la sesión existente; no la reinicia ni cambia su rutina original.
La planificación semanal puede cambiar o la rutina archivarse sin alterar esta copia.

`GET /api/training/daily` añade `session` a su respuesta anterior. Si existe, el frontal da prioridad
a la sesión sobre la planificación. Los días sin sesión muestran el plan semanal actual; el plan
no guarda versiones históricas. No se inventan sesiones a partir de asignaciones antiguas.

### Guardar resultados

`PUT /api/training/sessions/{id}` recibe:

```json
{
  "write_id": "UUID nuevo por edición; reutilizar al reintentar",
  "revision": 0,
  "complete": false,
  "exercises": [
    {
      "id": "UUID de session_exercises devuelto por la API",
      "status": "completed",
      "sets": [
        { "reps": 10, "load_kg": 40, "duration_seconds": null },
        { "reps": 8, "load_kg": 42.5, "duration_seconds": null }
      ]
    }
  ]
}
```

Los UUID del ejemplo son marcadores, no valores para enviar. Se exige incluir exactamente todos
los ejercicios de la sesión, sin repetirlos ni añadir identificadores de otra sesión. Se conservan
su orden y sus identidades; el cliente solo cambia el estado y las series.

- Estado del ejercicio: `pending`, `completed` o `skipped`. La sesión permanece `in_progress`
  con `complete = false`. Para completar, todos los ejercicios deben estar resueltos y al menos
  uno realizado. Una sesión completa puede corregirse enviando `complete = true`.
- Cada ejercicio admite de 1 a 100 series. Fuerza completada exige de 1 a 1000 repeticiones y una
  carga explícita por serie, con hasta tres decimales. `0` significa sin carga externa; `null` no es 0.
  Cardio/movilidad completados exigen una duración de 1 a 86400 segundos por serie.
- Las series pendientes pueden contener valores previstos o incompletos. Las omitidas se guardan
  como `skipped`, con las métricas nulas. **Las estadísticas deben filtrar series completadas**.
- El guardado reemplaza las series dentro de una transacción, manteniendo los UUID de los ejercicios
  de sesión. Los UUID de las series no son un contrato estable para referencias externas.
- Una revisión desactualizada responde 409, evitando perder una edición realizada en otro equipo.
  Repetir el último `write_id` devuelve el registro vigente sin volver a escribir. Tras otra edición,
  un reintento antiguo recibe conflicto; el cliente debe recargar. No reutilizar un identificador
  de escritura para contenidos diferentes.

La fecha de calendario es explícita. La zona horaria procede del perfil. Si esa fecha coincide con
el día actual del perfil, `started_at` registra el inicio; en registros de otras fechas se usa mediodía
local y `time_is_estimated = true`. `finished_at` y `completed_at` reflejan la confirmación del registro,
no prueban la hora a la que se realizó un entrenamiento histórico. No calcular duración real de
sesiones retroactivas a partir de esos timestamps. La UI deshabilita registrar fechas futuras.

### Datos disponibles para la futura evolución

- `workout_sessions.user_id`, `local_date`, `routine_id`, `name_snapshot`, `status`.
- `session_exercises.exercise_id`: identidad del catálogo para agrupar el mismo ejercicio a lo largo
  del tiempo. `exercise_name_snapshot` conserva el nombre que tenía en cada sesión.
- `workout_sets`: número de serie, repeticiones, carga y duración reales, con estado explícito.
- Las consultas pueden obtener carga máxima, repeticiones, series y volumen por fecha y ejercicio.
  Deben usar solo sesiones y series completadas. Comparar cambios de modalidad o ejercicio requiere
  reglas explícitas; no agrupar exclusivamente por un nombre que puede cambiar.

La página `/training/progress` muestra el historial de sesiones completadas y gráficas por ejercicio.
El calendario de la home permite consultar y corregir cada registro. Las sesiones libres y varias
sesiones diarias desde la home siguen fuera de este caso de uso.

### API de progreso

`GET /api/training/progress` recibe `from`, `to` (fechas estrictas `YYYY-MM-DD`) y `exercise_id`
opcional. Rechaza rangos invertidos o de más de 366 días inclusivos. El usuario procede del token.

La respuesta contiene:

- `sessions`: sesiones diarias completadas, de más reciente a más antigua, con fecha, nombre copiado,
  número de ejercicios y series realizados, repeticiones de fuerza, volumen y duración acumulada.
- `exercises`: identidades de ejercicios con series completadas en el período. El nombre y la modalidad
  proceden de su snapshot más reciente. Archivar la rutina o el ejercicio no elimina sus registros.
- `selected_exercise_id`: el ejercicio solicitado si tiene datos en el período, o el primero por nombre.
  Si el período no contiene ejercicios, devuelve `null` y una lista de puntos vacía.
- `points`: una fila por sesión y ejercicio, ordenada por fecha, con series, repeticiones, carga máxima,
  volumen y duración. Si el ejercicio aparece varias veces en una sesión se combinan sus series.

Las tres consultas se ejecutan bajo una misma lectura consistente. Solo se suman series `completed`
de ejercicios `completed` en sesiones `completed` de la home (`is_daily`). Las consultas usan el índice
existente por usuario/fecha y agregan en PostgreSQL; no cargan todas las series ni hacen una consulta
por sesión. La limitación de una sesión diaria acota las respuestas a 366 sesiones/puntos por rango.

La carga máxima conserva `NULL` cuando no aplica; `0 kg` representa una carga registrada de cero.
El volumen es la suma de carga externa × repeticiones en ejercicios de fuerza, no una medida médica
ni una estimación de 1RM. La duración acumula únicamente las series de cardio/movilidad. Las medias
y diferencias deben explicitar qué variable y período comparan. Esta funcionalidad no necesita
una migración adicional a `0008`.

## Agua

- `GET /api/water/daily?date=YYYY-MM-DD` devuelve `date`, `goal_ml`, `total_ml` y `entries`, ordenadas
  por creación e identificador. El porcentaje es `100 × total_ml / goal_ml` y puede superar 100.
- `POST /api/water/intakes` recibe UUID, fecha y cantidad entera entre 1 y 5000 ml. Un reintento con
  el mismo UUID y contenido devuelve el día sin duplicar el aporte. Reutilizarlo con otros datos,
  otro usuario o tras deshacerlo produce 409.
- `DELETE /api/water/intakes/{id}` marca un aporte propio como eliminado, de forma idempotente.
  Los aportes eliminados quedan excluidos de totales. La home ofrece deshacer el último.
- `PUT /api/water/goal` recibe fecha y objetivo de 100 a 10000 ml. El objetivo se aplica desde esa
  fecha hasta la siguiente versión. Sin versión aplicable, el objetivo por defecto es 2000 ml.
  Es una configuración del usuario; no es una recomendación personalizada.
- Como límites operativos, el backend admite hasta 1000 aportes activos y 50000 ml por día.
  Los límites no representan recomendaciones de consumo.

Las escrituras se serializan por usuario. UUID e índice por usuario/fecha permiten un registro rápido
y seguro ante reintentos. En el frontal, una respuesta incierta conserva el identificador para
reintentar; recargar o salir pierde ese intento local y requiere consultar los aportes antes de repetirlo.

### Estadísticas futuras

Para una semana o un mes, generar primero todos los días del período y unir los aportes activos por
usuario/fecha. Los días sin aportes tienen consumo 0, no deben desaparecer del denominador de la media.
Seleccionar para cada día la última versión de objetivo con `effective_from <= fecha`, o 2000 ml.
Esto permite obtener consumo y porcentaje diario, media semanal y mensual, preservando los objetivos
anteriores al cambiar el actual. Un día sin registro significa **0 registrado**, no evidencia de que
la persona no bebiera agua. Las gráficas futuras deben comunicar esa distinción.

## Comprobaciones de esta entrega

Backend y frontend compilan; Clippy y Oxlint no indican errores. La migración 0008 está aplicada en
el entorno local. No se han ejecutado tests ni completado una verificación interactiva del flujo.
No se han creado datos de ejemplo en la base del usuario ni desplegado estos cambios en Raspberry.
