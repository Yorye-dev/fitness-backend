# Refactor RF-008 a RF-020

Punto de partida: RF-001 a RF-007 ya estaban aplicadas en develop.

| Issue de GitHub | Implementación |
| --- | --- |
| [RF-008](https://github.com/Yorye-dev/fitness-backend/issues/12) | application/security/token_service.rs; JwtTokenService en infrastructure/auth; Login, Register, Refresh y Verify reciben el contrato |
| [RF-009](https://github.com/Yorye-dev/fitness-backend/issues/13) | PasswordService y Argon2PasswordService; Login, Register y ChangePassword usan la abstracción |
| [RF-010](https://github.com/Yorye-dev/fitness-backend/issues/14) | Entidades sin SQLx ni Serde; User y SignInUser en domain/user/entity.rs; UserResponse es la salida pública |
| [RF-011](https://github.com/Yorye-dev/fitness-backend/issues/15) | infrastructure/persistence/models y mappers para User, SignInUser, Goals, Meal, Consumption y agregados |
| [RF-012](https://github.com/Yorye-dev/fitness-backend/issues/16) | RepositoryError en los contratos; traducción de SQLx en infrastructure/persistence/errors.rs |
| [RF-013](https://github.com/Yorye-dev/fitness-backend/issues/17) | DomainError para negocio, ApplicationError para orquestación y ApiError para HTTP |
| [RF-014](https://github.com/Yorye-dev/fitness-backend/issues/18) | Sex, ActivityLevel y Goal en domain/user; UnitType en domain/nutrition; eliminado domain/enums |
| [RF-015](https://github.com/Yorye-dev/fitness-backend/issues/19) | Nutrition contiene daily, goals y CRUD de alimentos; se elimina set_goals y update_goals deja User |
| [RF-016](https://github.com/Yorye-dev/fitness-backend/issues/20) | Handlers auth, user y nutrition separados por función, todos conectados a rutas |
| [RF-017](https://github.com/Yorye-dev/fitness-backend/issues/22) | routes/auth y routes/protected/{user,nutrition}; middleware único en el árbol protegido |
| [RF-018](https://github.com/Yorye-dev/fitness-backend/issues/23) | Módulos en lib.rs; main.rs limitado a bootstrap y cierre del servidor |
| [RF-019](https://github.com/Yorye-dev/fitness-backend/issues/24) | Tests HTTP de login, refresh, registro y /api/me; errores uniformes e identidad del token |
| [RF-020](https://github.com/Yorye-dev/fitness-backend/issues/25) | DailyNutritionQuery -> handler -> GetDailyNutritionUseCase -> NutritionRepository -> DailyNutritionResponse |

## Decisiones

- Los detalles de JWT, Argon2, PostgreSQL y HTTP quedan en sus adaptadores.
- Los casos de uso comparten dependencias mediante Arc. Los repositorios conservan contratos
  separados para perfil, objetivos, alimentos e ingestas.
- User incluye el hash porque autenticación lo necesita; no implementa Serialize.
- Los tipos de enum de PostgreSQL están en los rows. Los enums del dominio no conocen el driver.
- Las altas y actualizaciones que afectan perfil y objetivos se confirman en una transacción.
- El cálculo de objetivos se centraliza en NutritionCalculator. Los objetivos personalizados
  se gestionan con UpdateGoalsUseCase y se consultan desde el resumen diario.
- Los extractores convierten errores JSON, query y path al contrato ApiError.
- Se mantienen las migraciones existentes: este refactor no cambia el esquema.
- User vive en entity.rs para evitar el nombre repetido user::user señalado por Clippy.
- Los handlers de alimentos y objetivos implementan la estructura prevista en RF-015/016/017.
  La creación de tablas de ejercicios y nuevos flujos de entrenamiento quedan para sus propias issues.

## Evidencia automatizada

| Archivo de tests | Cobertura |
| --- | --- |
| tests/architecture.rs | Domain y Application sin dependencias de infraestructura o HTTP |
| tests/security.rs | Firma, expiración, UUID, duración de tokens y Argon2 |
| tests/auth_http.rs | Éxito/error de login, refresh, registro, perfil, contraseña y errores HTTP |
| tests/cors.rs | Orígenes de desarrollo y configuración de producción con proxy |
| tests/nutrition_http.rs | Día vacío, totales, objetivos, fechas, identidad y CRUD de alimentos |
| tests/persistence.rs | Rollback, errores internos ocultos, rows, paginación, estadísticas y propiedad |
| tests/user_profile.rs | Límites del perfil, actualización de la entidad y hashes ocultos en Debug |
| tests/health_http.rs | Readiness 204 con PostgreSQL disponible y 503 con pool cerrado |

Las pruebas de integración usan bases PostgreSQL temporales independientes. Los comandos de
formato, compilación, Clippy y tests están en el README.

## Comprobaciones del refactor RF-008 a RF-020

- cargo fmt --check: correcto.
- cargo check --locked: correcto.
- cargo clippy --all-targets --locked -- -D warnings: correcto.
- cargo test --locked -- --test-threads=2: 19 tests correctos.
- Imagen Docker de producción: compilada y arrancada en Docker Desktop (Linux/amd64).
- Readiness de la imagen de producción: 204; endpoint diario sin token: 401.

La Raspberry física no se ha utilizado para estas comprobaciones. La implementación está preparada
en una rama local; la publicación y el cierre de las issues en GitHub se gestionan al integrar los cambios.

## Mejoras posteriores de la base arquitectónica

- UserProfile concentra la validación del perfil y mantiene sus campos privados. Registro,
  actualización y lectura de PostgreSQL utilizan el mismo constructor validado.
- User encapsula identidad, credenciales y perfil; update_profile solo admite un UserProfile válido.
  UserFactory recibe datos con un perfil ya validado. User y SignInUser ocultan su hash en Debug.
- UserRow se convierte al dominio con TryFrom. Los errores de datos persistidos se traducen
  a RepositoryError::Unexpected. En escrituras, la conversión se completa antes del commit.
- Presentation expone los constructores de rutas y CORS. Sus handlers, DTO, extractores y middleware
  son privados. Los rows, mappers y claims de Infrastructure también son privados.
- AppState mantiene sus casos de uso visibles únicamente dentro del crate. AppDependencies conserva
  su API pública de inyección mediante interfaces.
- ReadinessCheck define el contrato de disponibilidad en Application. PostgresReadinessCheck
  implementa la consulta y su timeout en Infrastructure. El handler conserva las respuestas 204/503.
- El README incluye un flujo para añadir casos de uso respetando estas reglas.

Comprobaciones iniciales de esta ampliación:

- cargo fmt --check: correcto.
- cargo check --all-targets --locked: correcto, incluida la compilación de los tests existentes.
- cargo clippy --all-targets --locked -- -D warnings: correcto.
- Backend de desarrollo reiniciado y declarado healthy por Docker Compose.
- En esa pasada no se ejecutó la suite de tests; los 19 tests indicados arriba corresponden
  a la comprobación anterior del refactor RF-008 a RF-020.

## Separación de tokens y comprobación final

- TokenService expone validate_access_token y validate_refresh_token; se elimina la validación
  pública genérica. VerifyTokenUseCase acepta acceso y RefreshTokenUseCase acepta renovación.
- Los JWT llevan token_type firmado (access/refresh). Falta de tipo, valores desconocidos,
  tipo incorrecto, firma inválida, expiración y UUID inválido producen TokenError::Invalid.
- HS256 se declara explícitamente tanto al firmar como al validar. Se conservan subject,
  exp, los tiempos de vida y el formato de las respuestas HTTP.
- Los tokens previos sin token_type dejan de ser válidos: el usuario debe volver a iniciar sesión.
- Se amplían las pruebas de seguridad y HTTP, incluyendo tokens intercambiados y antiguos.
  También se cubren las mejoras de UserProfile, rollback ante una fila inválida y ReadinessCheck.

Verificación final de esta versión realizada en Docker Desktop, con PostgreSQL y bases temporales
por test. El bloqueo inicial de revisión automática quedó resuelto antes de ejecutar los comandos:

- cargo fmt --check: correcto.
- cargo check --all-targets --locked: correcto.
- cargo clippy --all-targets --locked -- -D warnings: correcto.
- cargo test --locked -- --test-threads=2: 30 tests correctos, sin fallos ni tests ignorados.
- git diff --check: correcto.

Desglose: arquitectura (1), autenticación HTTP (9), CORS (2), readiness (1), nutrición HTTP (5),
persistencia (4), seguridad (5) y perfil de dominio (3).

## Migraciones posteriores del modelo nutricional y de entrenamiento

Se incorporan las migraciones 0002–0004 y se adaptan los repositorios a las nuevas tablas.
El contrato HTTP sigue disponible. La prueba de persistencia de perfiles inválidos ahora comprueba
que PostgreSQL rechaza el dato antes de almacenarlo. La suite completa mantiene 30 tests correctos.
Los detalles de ejecución, compatibilidad y copia previa están en [database/migrations.md](database/migrations.md).
