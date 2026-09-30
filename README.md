# Fitness Backend

API de nutrición en Rust, Axum y PostgreSQL. La configuración de Docker para Windows, Linux y
Raspberry Pi vive en el repositorio hermano **fitness-deploy**.

## Arranque

Coloca fitness-backend, fitness-front y fitness-deploy como carpetas hermanas. Desde fitness-deploy:

- Windows: powershell -ExecutionPolicy Bypass -File .\start.ps1
- Linux: bash ./start.sh development
- Raspberry Pi de 64 bits: bash ./start.sh production

El backend aplica migrations/ antes de aceptar peticiones. La base se crea vacía en el primer
arranque. Los siguientes arranques aplican solo las migraciones pendientes y conservan los datos.
Los SQL ya aplicados no se editan: cada cambio de esquema se añade en un archivo nuevo.

## Arquitectura

| Capa | Responsabilidad |
| --- | --- |
| domain | Entidades, enums, cálculo nutricional, contratos de repositorios y errores de negocio/persistencia |
| application | Casos de uso y contratos TokenService / PasswordService / ReadinessCheck |
| infrastructure | JWT, Argon2 y PostgreSQL; rows, mappers y comprobación de disponibilidad |
| presentation | Rutas, middleware, extractores, DTOs, contrato de respuestas y errores |
| app_state | Construcción e inyección de las implementaciones |
| config | Lectura de variables, archivos secretos y conexión |
| main | Configuración, pool, migraciones, Router, servidor y cierre ordenado |

Domain y Application no importan Axum, Serde ni SQLx. Application utiliza interfaces compartidas
mediante Arc; AppState::from_dependencies permite sustituir sus implementaciones. El crate de
librería permite construir el Router en tests sin abrir un puerto.

Las contraseñas usan Argon2 y se procesan fuera de los trabajadores asíncronos. La implementación
JWT conserva los claims subject/exp y la duración de 60 minutos / 7 días, y añade el claim firmado
token_type con valor access o refresh. La firma se valida exclusivamente con HS256. TokenService
ofrece validate_access_token y validate_refresh_token: /api solo admite access y /auth/refresh
solo admite refresh. El middleware entrega a los handlers únicamente AuthenticatedUser con un UUID validado.

Los tokens anteriores sin token_type se rechazan con 401 INVALID_TOKEN. Tras desplegar este cambio
hay que volver a iniciar sesión. Las respuestas JSON de login y renovación mantienen su formato.

Las escrituras del perfil y sus objetivos nutricionales se realizan en una transacción.
Las consultas de recursos personales utilizan el identificador del usuario autenticado.

### Reglas y encapsulación

- UserProfile::new valida peso, altura y edad en Domain. Conserva los límites existentes:
  peso finito mayor que 0 y hasta 300 kg, altura de 1 a 250 cm y edad de 1 a 120 años.
- User y UserProfile tienen campos privados. El registro recibe un perfil validado;
  User::update_profile sustituye el perfil completo por otro válido. No hay setters de valores sueltos.
- La lectura desde PostgreSQL también construye UserProfile. Una fila con un perfil inválido
  produce un error interno de persistencia; no se presenta como un error de validación del cliente.
- Los hashes de User y SignInUser se ocultan en Debug y siguen excluidos de los DTO HTTP.
- Los handlers, DTO, extractores, middleware, rows, mappers y claims son detalles internos.
  Permanecen públicos los casos de uso, los contratos, las entidades y los adaptadores necesarios
  para componer la aplicación. Los campos de AppState solo son accesibles dentro del crate.
- /health/ready recibe ReadinessCheck. PostgresReadinessCheck ejecuta la consulta con un límite
  de dos segundos en Infrastructure; Presentation transforma el resultado en 204 o 503.

### Añadir un caso de uso

1. Define las reglas y tipos del negocio en domain/<funcionalidad>. Protege las invariantes
   con campos privados y constructores o métodos que validen antes de modificar la entidad.
2. Crea el caso de uso en application/<funcionalidad>. Recibe interfaces mediante Arc y
   devuelve resultados de aplicación o dominio, sin tipos HTTP ni SQLx.
3. Implementa el acceso a PostgreSQL en infrastructure/persistence. Mantén los rows internos,
   valida su conversión al dominio y usa una transacción para las escrituras que deban ser atómicas.
4. Añade el DTO, el handler y la ruta en Presentation. Obtén el usuario desde AuthenticatedUser
   y mantén el contrato común de respuestas y errores.
5. Conecta las dependencias en AppState::from_dependencies. Expón únicamente los elementos
   que deban utilizarse desde fuera del crate; utiliza mod privado o pub(crate) para los demás.

## API

Todas las respuestas con contenido utilizan uno de estos contratos:

- Éxito: {"data": ...}
- Listado paginado: {"data": [...], "meta": {"pagination": ...}}
- Error: {"error": {"code": "...", "message": "..."}}
- Las operaciones sin contenido devuelven 204.

| Método | Ruta | Uso |
| --- | --- | --- |
| POST | /auth/register | Registrar cuenta y calcular objetivos iniciales |
| POST | /auth/sign_in | Obtener access_token y refresh_token |
| POST | /auth/refresh | Renovar access_token |
| GET | /api/me | Consultar el usuario autenticado |
| PUT | /api/me | Actualizar perfil y recalcular objetivos |
| PUT | /api/me/password | Cambiar contraseña |
| GET | /api/nutrition/daily?date=YYYY-MM-DD | Consultar ingestas y objetivos del día |
| GET | /api/nutrition/goals | Consultar objetivos nutricionales |
| PUT | /api/nutrition/goals | Establecer objetivos nutricionales personalizados |
| GET | /api/nutrition/meals?page=1&per_page=20&q=arroz | Listar y buscar alimentos propios |
| POST | /api/nutrition/meals | Crear alimento |
| GET / PUT / DELETE | /api/nutrition/meals/{id} | Consultar, editar o eliminar alimento propio |
| POST | /api/nutrition/consumptions | Registrar una ingesta propia por gramos o porciones y fecha |
| PUT | /api/nutrition/consumptions/{id} | Editar cantidad, porciones o fecha de una ingesta propia |
| DELETE | /api/nutrition/consumptions/{id} | Eliminar una ingesta propia y descontarla del día |
| GET | /health/ready | Comprobar disponibilidad de PostgreSQL, 204 o 503 |

El árbol /api requiere Authorization: Bearer <token>. Auth y readiness son públicos.
El usuario de las operaciones se obtiene del token. Los DTOs de Nutrition rechazan un user_id
enviado por el cliente. La paginación admite páginas desde 1 y entre 1 y 100 elementos.

El endpoint diario exige una fecha real en formato YYYY-MM-DD. Un día sin ingestas devuelve
200, cantidades consumidas a cero y meals vacío. Los totales se suman a partir de los valores
guardados en las ingestas; los objetivos se leen de PostgreSQL o se calculan a partir del perfil
si todavía no existen. No se crean ingestas de ejemplo.

Las respuestas de usuario mantienen los valores públicos existentes de los enums, por ejemplo
Male, ModeratelyActive y Maintain. Las requests mantienen male, moderately_active y maintain.
Ninguna respuesta de usuario contiene password_hash.

Los errores de validación devuelven 422, las credenciales/tokens inválidos 401, los recursos
ausentes 404 y los conflictos 409. Los fallos internos devuelven un mensaje genérico.

Las rutas `/api/nutrition/meals` conservan su contrato, pero utilizan la tabla `foods`.
Eliminar un alimento lo archiva: las ingestas anteriores conservan su nombre y nutrientes.
Los objetivos se guardan por fecha y el resumen busca la versión vigente en el día solicitado.
Si no existe una versión para ese día, se conserva el cálculo estimado desde el perfil que ya
ofrecía la API; no debe interpretarse como un objetivo histórico registrado.

## Comidas reutilizables e ingestas

Crear una comida en `/api/nutrition/meals` guarda su nombre y nutrientes **por 100 g o por unidad** en el catálogo
personal; no suma consumo. Cada uso se registra en `/api/nutrition/consumptions` con un UUID nuevo:

```json
{
  "id": "891ced7e-d7a7-41ea-8b4c-a387055ffdef",
  "meal_id": "a765c563-8ba1-4556-909f-1e84b577f1ee",
  "date": "2026-09-30",
  "quantity_grams": 150
}
```

`meal_id` debe identificar una comida propia y activa. El usuario se obtiene del access token.
Se admite una fecha real `YYYY-MM-DD` y una cantidad finita entre 0,001 y 1.000.000 g; PostgreSQL
almacena gramos con tres decimales. El cliente no puede enviar totales de nutrientes ni `user_id`.

La operación copia nombre y nutrientes al registro, y PostgreSQL genera cada aporte como
`cantidad_en_gramos × valor_por_100g / 100`. El resumen suma las ingestas de la fecha y mantiene
los objetivos separados: **restante = objetivo − consumido**. Eliminar una ingesta actualiza el
consumo; editar o archivar una comida no modifica las ingestas anteriores.

El `id` identifica una ingesta, no la comida del catálogo. Mientras la ingesta exista, repetir el
mismo `id`, usuario, fecha, comida y cantidad devuelve el registro existente sin duplicar totales
(201). Cambiar sus datos con el mismo `id` devuelve 409. Se serializan los reintentos concurrentes
mediante un bloqueo transaccional. Una ingesta nueva necesita un UUID nuevo, aunque se repita comida.
La eliminación no mantiene una reserva del UUID: un POST posterior puede volver a crearlo.

La búsqueda `q` se aplica al nombre, sin distinguir mayúsculas/minúsculas, sobre el catálogo propio
no archivado, antes de paginar. Su longitud máxima es 200 caracteres; no admite comodines.
### Editar ingestas y registrar porciones

`PUT /api/nutrition/consumptions/{id}` recibe la fecha y la cantidad completa que sustituirán a las
actuales. No cambia el alimento del catálogo ni los valores nutricionales originales. Puede corregir
ingestas de hoy o de cualquier día anterior, también si el alimento se ha archivado.

Tanto POST como PUT aceptan **una** de estas formas de cantidad:

- `quantity_grams`: gramos totales, entre 0.001 y 1000000 y hasta tres decimales.
- `portion_count` y `portion_grams`: unidades (admite fracciones) y gramos por unidad, con los mismos
  límites individuales. El servidor multiplica ambos y redondea el peso total a 0.001 g. El total
  también debe estar entre 0.001 y 1000000 g. No enviar `quantity_grams` junto con las porciones.
- Solo `portion_count`: para comidas guardadas por unidad. No enviar gramos; se admiten fracciones
  y el mismo intervalo de 0.001 a 1000000, con tres decimales.

Por ejemplo, el cuerpo de un PUT para dos galletas de 29 g es:

```json
{ "date": "2026-09-30", "portion_count": 2, "portion_grams": 29 }
```

La respuesta contiene 58 g y conserva las dos propiedades de porción. POST añade a ese cuerpo `id`
y `meal_id`. Los registros por gramos devuelven las propiedades de porción como `null`.

Las modificaciones se realizan en una transacción y se filtran por usuario. Mover una ingesta
recalcula los días de origen y destino sin mover otros alimentos que compartan su cabecera.
Los nutrientes se recalculan desde el snapshot; en entradas `legacy`, desde la proporción de los
totales históricos (con su precisión original REAL). Nunca se usan los nutrientes actuales del
catálogo para reescribir el pasado. Los PUT repetidos con los mismos datos no suman nuevas ingestas;
si hay ediciones simultáneas se conserva la última escritura, sin control de versión optimista.

La migración `0005_consumption_portions.sql` añade las porciones opcionales y sus restricciones,
sin alterar los valores de las ingestas existentes. Se aplica automáticamente al arrancar.

### Comidas por unidad (batidos, platos o envases completos)

El catálogo admite `nutrition_basis: "per_unit"` y nutrientes de una unidad completa:

```json
{
  "name": "Batido de proteínas",
  "nutrition_basis": "per_unit",
  "calories_per_unit": 150,
  "protein_per_unit": 25,
  "carbs_per_unit": 7,
  "fat_per_unit": 2
}
```

Son valores ilustrativos, que debe sustituir el usuario por los de su batido. Para consumirlo,
POST envía `id`, `meal_id`, `date` y `portion_count: 1`. PUT conserva el contrato sin los dos UUID
del cuerpo. No se necesitan gramos: los nutrientes se calculan como unidades × valor por unidad.
En estas ingestas `quantity_grams` y `portion_grams` son `null`; el historial muestra unidades.

El contrato anterior `*_per_100g` sigue disponible, con `nutrition_basis: "per_100g"` o sin ese campo.
Los cuatro nutrientes deben corresponder a la base elegida; las respuestas incluyen ambas familias
y la que no se utiliza vale `null`. El modo por 100 g limita macros a 100 g; por unidad admite hasta
menos de 100000 g (una comida completa puede contener más de 100 g de un macro).

La migración `0006_unit_based_foods.sql` conserva la base y los nutrientes como snapshot en cada
ingesta. Editar una comida o cambiar su base no convierte ni recalcula registros anteriores.
Una petición de cantidad incompatible con la base actual (o con el snapshot al editar) devuelve 409.
Los alimentos y las ingestas existentes mantienen su base por 100 g.

## Rutinas y planificación semanal

Las rutas de entrenamiento requieren un access token y operan sobre los datos de su usuario:

| Método | Ruta | Uso |
| --- | --- | --- |
| GET | `/api/training/routines` | Listar rutinas activas con sus ejercicios ordenados |
| PUT | `/api/training/routines/{id}` | Crear o reemplazar una rutina con UUID del cliente |
| DELETE | `/api/training/routines/{id}` | Archivar una rutina y liberar sus asignaciones semanales |
| GET / PUT | `/api/training/week` | Consultar o reemplazar la semana recurrente |
| GET | `/api/training/daily?date=YYYY-MM-DD` | Obtener la rutina del día de la semana correspondiente |

Las respuestas usan `{ data: ... }`; DELETE responde 204. La semana tiene siete entradas en `days`,
con `weekday` de 1 (lunes) a 7 (domingo) y `routine_id` propio y activo o `null` para descanso.
Una rutina puede asignarse a varios días. El plan actual no está versionado por fechas: consultar
una fecha pasada devuelve la asignación semanal vigente, no un entrenamiento realizado.

El cuerpo de rutina contiene `name`, `description` y `exercises`. Cada ejercicio incluye nombre,
modalidad (`strength`, `cardio` o `mobility`), series, descanso y notas. Fuerza requiere un rango de
repeticiones y admite carga opcional; cardio y movilidad requieren duración y excluyen carga y
repeticiones. El dominio valida límites, valores finitos y compatibilidad de métricas. Se permiten
hasta 100 rutinas activas por usuario y entre 1 y 50 ejercicios por rutina.

Los cambios de rutinas y semana se guardan en transacciones, serializadas por usuario para evitar
asignar una rutina mientras se archiva. Reenviar el PUT con el mismo UUID y contenido conserva una
sola rutina. Las ediciones concurrentes conservan el último guardado. Las consultas usan filtros de
propiedad y las claves foráneas incluyen `user_id`. Archivar conserva posibles sesiones anteriores.
El registro de sesiones y series realizadas sigue pendiente; las métricas actuales son objetivos.

## Comprobaciones

Desde fitness-deploy, con el entorno de desarrollo arrancado:

    docker compose --env-file .env.development exec -T backend cargo fmt --check
    docker compose --env-file .env.development exec -T backend cargo check --locked
    docker compose --env-file .env.development exec -T backend cargo clippy --all-targets --locked -- -D warnings
    docker compose --env-file .env.development exec -T backend cargo test --locked -- --test-threads=2

La imagen de desarrollo incluye rustfmt y Clippy. Si vienes de una versión anterior,
vuelve a ejecutar start.ps1 o start.sh para reconstruirla.

Los tests HTTP usan Router::oneshot y PostgreSQL real. SQLx crea una base temporal por test,
aplica las migraciones y la elimina cuando pasa. No requieren arrancar el servidor HTTP.
DATABASE_URL debe apuntar a un servidor local de pruebas con permiso para crear bases
(el usuario de PostgreSQL del entorno de desarrollo ya lo tiene). No uses credenciales de producción.
Si un test falla, SQLx puede conservar su base temporal para diagnosticarlo.

También puedes arrancar solo PostgreSQL y ejecutar los tests en un contenedor efímero:

    docker compose --env-file .env.development up -d db
    docker compose --env-file .env.development run --rm --no-deps backend cargo test --locked -- --test-threads=2

La suite cubre contratos HTTP, aislamiento de usuarios, fechas, días vacíos, totales,
mapeos PostgreSQL, transacciones, contraseñas y límites entre capas. Incluye el intercambio de tipos
de token, rechazo de tokens antiguos, validación del perfil y disponibilidad de PostgreSQL.

## Configuración

La configuración de ejecución se proporciona desde fitness-deploy:

- DATABASE_URL o DATABASE_URL_FILE: conexión PostgreSQL.
- SECRET_KEY o SECRET_KEY_FILE: secreto de firma JWT.
- BIND_ADDRESS: socket de escucha; en Docker, 0.0.0.0:8080.
- APP_URL: compatibilidad temporal con el nombre anterior.
- PROJECT_NAME: nombre del proceso.
- CORS_ALLOWED_ORIGINS: lista de orígenes exactos separados por comas.

Las variantes _FILE tienen prioridad. Los secretos y .env reales quedan fuera de Git.
En producción, el frontal y la API comparten origen mediante Nginx; CORS puede quedar vacío.

Consulta [la trazabilidad del refactor](docs/refactor.md) para relacionar cada issue con el código.

## Esquema y migraciones

El [esquema de base de datos](docs/database/README.md) incluye diagramas y reglas de negocio.
Las migraciones activas son:

| Versión | Cambios |
| --- | --- |
| 0001 | Esquema inicial, conservado sin modificaciones |
| 0002 | Restricciones de usuario, zona horaria, timestamps y catálogo `foods` |
| 0003 | Diario, versiones de objetivos y conservación del esquema antiguo en `legacy` |
| 0004 | Ejercicios, rutinas, sesiones y series de entrenamiento |
| 0005 | Porciones opcionales en ingestas nuevas e históricas, y ampliación de la vista de lectura |
| 0006 | Catálogo y snapshots por unidad, gramos opcionales y consumos generados según la base |
| 0007 | Plan semanal recurrente por usuario, con una rutina opcional por día |

El backend aplica las migraciones pendientes al arrancar, tanto en Windows/Linux como en Raspberry.
Se registra cada versión y su checksum en `_sqlx_migrations`. El [DDL de referencia](docs/database/schema.sql)
sirve para leer el modelo; la instalación y la actualización del backend utilizan `migrations/`.

Las escrituras actuales ya usan el nuevo esquema nutricional. Entrenamiento permite gestionar
rutinas y su planificación semanal; el registro de sesiones y series se desarrollará después. PostgreSQL almacena los
decimales y calcula los consumos con `NUMERIC`; los adaptadores mantienen por compatibilidad los
tipos `f32` actuales de nutrientes; cantidades y porciones se exponen como `f64`. La conversión completa del dominio a decimales queda separada
de esta actualización de esquema.

Los datos antiguos se mantienen en `legacy` y se consultan junto a los nuevos a través de
`consumption_entries`. No se reconstruyen sus nutrientes desde el catálogo actual.
Consulta [el procedimiento de actualización y recuperación](docs/database/migrations.md).
