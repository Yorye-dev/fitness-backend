# Migraciones de nutrición y entrenamiento

## Estado

Las migraciones `0001` a `0004` están aplicadas en el PostgreSQL de desarrollo de Windows.
La ejecución se hizo con el arranque del backend y quedó registrada en `_sqlx_migrations`.
La versión `0001_initial_schema.sql` se conserva sin modificaciones.

| Migración | Contenido |
| --- | --- |
| `0002_users_and_foods.sql` | Nuevas columnas y restricciones de usuarios; catálogo decimal de alimentos y copia desde `meals` |
| `0003_nutrition_diary.sql` | Diario, snapshots, consumos generados, objetivos por fecha y traslado de tablas antiguas a `legacy` |
| `0004_workout_tracking.sql` | Ejercicios, rutinas, ejercicios planificados, sesiones, ejercicios de sesión y series |

El resultado contiene 11 tablas de negocio en `public`, 3 tablas históricas en `legacy`, las vistas
`consumption_entries` / `daily_nutrition_totals` y la tabla interna `_sqlx_migrations`.

## Actualización en Windows, Linux o Raspberry

1. Guardar una copia de la base antes de actualizar una instalación que tenga datos.
2. Obtener esta misma versión del repositorio backend en el equipo de destino.
3. Arrancar con `start.ps1` en Windows o `bash ./start.sh development` en Linux, desde `fitness-deploy`.
   En la Raspberry se usa `bash ./start.sh production` con sus variables y secretos de producción.
4. El nuevo backend conecta a la base configurada y ejecuta únicamente las migraciones pendientes.
   La configuración de readiness permite confirmar cuándo vuelve a aceptar peticiones.

No ejecutar `docs/database/schema.sql` sobre la base existente ni borrar `_sqlx_migrations`.
SQLx ejecuta cada migración dentro de su propia transacción y registra su checksum. Si una migración
falla, las anteriores que ya terminaron permanecen aplicadas; se debe resolver la causa y reanudar.
Una migración que ya se haya aplicado no debe editarse: las correcciones se añaden en otra versión.

## Datos anteriores

- Los usuarios permanecen en `public.users`. Su peso se convierte a decimal de tres posiciones.
- Los alimentos se copian a `foods`, conservando sus UUID. Los valores por 100 g pasan a decimal.
- Las tres tablas anteriores se mueven a `legacy`, conservando los consumos originales y sus referencias.
- Las consultas del backend combinan consumos nuevos y antiguos a través de `consumption_entries`.
  La eliminación explícita de una ingesta también puede eliminar una entrada histórica propia.
- Los objetivos antiguos se importan como una versión de origen `migration`. Su fecha de inicio se
  toma de la primera ingesta conocida o del día de migración; no representa un historial recuperado.
- Los datos incompatibles con las nuevas restricciones hacen fallar la migración para que se corrijan
  explícitamente. También se rechazan enlaces antiguos entre una ingesta y el alimento de otro usuario.

La base local estaba vacía al comenzar: 0 usuarios, alimentos, ingestas y objetivos. No se ha
eliminado ningún volumen ni se han creado cuentas o alimentos de ejemplo en el entorno real.

## Compatibilidad del backend

- Las rutas existentes mantienen sus nombres y respuestas. `meals` en la API corresponde a `foods`
  en la base. DELETE archiva el alimento y conserva las ingestas que lo referencian.
- El backend obtiene el nombre y los nutrientes de un alimento propio y activo al crear la ingesta.
  Los totales de entrada del contrato interno anterior se ignoran; PostgreSQL genera los consumos.
- El contrato de consumo que solo lleva fecha utiliza mediodía local y marca la hora como estimada.
- Los cambios de objetivos guardan la versión de hoy; varias correcciones de hoy sustituyen esa
  versión. Las versiones de días anteriores no se sobrescriben por esas operaciones.
- El resumen diario consulta la versión vigente para su fecha. Si no existe, mantiene el fallback
  estimado a partir del perfil del contrato anterior; esa estimación no es un dato histórico guardado.
- La persistencia usa NUMERIC. Los adaptadores conservan el f32 existente de dominio/DTO mediante
  conversiones explícitas; cambiar todo el dominio a tipos decimales es una tarea posterior.
- El esquema de entrenamiento está listo. Todavía no se han implementado sus casos de uso ni rutas.

## Copia previa y recuperación

Antes de aplicar las migraciones se detuvo brevemente el backend y se creó un dump con `pg_dump -Fc`:

`target/database-backups/pre-vnext-20260929-222440.dump`

Es una copia local de la base anterior, excluida de Git por estar dentro de `target/`. Conviene copiarla
a un destino de backups si se necesita conservarla a largo plazo; limpiar `target` puede borrarla.

Para recuperar una versión anterior, restaurar ese dump en una base separada usando `pg_restore`
y ejecutar el backend correspondiente a `0001` contra esa base. Comprobar la restauración antes de
cambiar la configuración del servicio. No se han creado migraciones automáticas de retroceso.

## Comprobaciones realizadas

- `cargo fmt --check`: correcto.
- `cargo check --all-targets --locked`: correcto.
- `cargo clippy --all-targets --locked -- -D warnings`: correcto.
- `cargo test --locked -- --test-threads=2`: 30 tests correctos sobre el nuevo esquema.
- `_sqlx_migrations`: versiones 1, 2, 3 y 4 con `success = true` en la base de desarrollo.
- Backend, PostgreSQL y frontend: saludables tras el arranque.

La suite crea bases temporales desde cero con todas las migraciones. La actualización local también
partió de tablas vacías. Estos resultados no acreditan una migración de historiales reales ni una
ejecución en ARM/Raspberry.
