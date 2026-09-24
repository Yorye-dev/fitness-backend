use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
}
