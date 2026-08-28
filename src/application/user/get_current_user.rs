use uuid::Uuid;

use crate::domain::user::{
    repository::UserRepository,
    user::User,
};

#[derive(Clone)]
pub struct GetCurrentUserUseCase<R>
where
    R: UserRepository,
{
    user_repository: R,
}

impl<R> GetCurrentUserUseCase<R>
where
    R: UserRepository,
{
    pub fn new(user_repository: R) -> Self {
        Self {
            user_repository,
        }
    }

    pub async fn execute(
        &self,
        user_id: Uuid,
    ) -> Result<Option<User>, sqlx::Error> {
        self.user_repository
            .get_user_by_id(&user_id)
            .await
    }
}
