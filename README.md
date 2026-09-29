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
| GET | /api/nutrition/meals?page=1&per_page=20 | Listar alimentos propios |
| POST | /api/nutrition/meals | Crear alimento |
| GET / PUT / DELETE | /api/nutrition/meals/{id} | Consultar, editar o eliminar alimento propio |
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
