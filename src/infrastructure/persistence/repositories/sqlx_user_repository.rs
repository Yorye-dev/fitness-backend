use crate::domain::{
    errors::RepositoryError,
    nutrition::goals::NutritionGoals,
    user::{
        entity::{SignInUser, User},
        repository::UserRepository,
    },
};
use crate::infrastructure::persistence::models::user_row::{
    ActivityLevelRow, GoalRow, SexRow, SignInUserRow, UserRow,
};
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct SqlxUserRepository {
    pool: PgPool,
}
impl SqlxUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for SqlxUserRepository {
    async fn save_user(
        &self,
        user: &User,
        goals: &NutritionGoals,
    ) -> Result<User, RepositoryError> {
        let mut transaction = self.pool.begin().await?;
        let row = sqlx::query_as::<_, UserRow>(
            "INSERT INTO users (id,username,password_hash,age,sex,height,weight,activity_level,goal)
             VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9) RETURNING *")
            .bind(user.id()).bind(user.username()).bind(user.password_hash()).bind(user.profile().age())
            .bind(SexRow::from(user.sex())).bind(user.profile().height()).bind(user.profile().weight())
            .bind(ActivityLevelRow::from(user.profile().activity_level())).bind(GoalRow::from(user.profile().goal()))
            .fetch_one(&mut *transaction).await?;
        sqlx::query("INSERT INTO users_nutrition_goals (id,user_id,protein_goal,carbs_goal,fats_goal,tdee,bmr)
                    VALUES ($1,$2,$3,$4,$5,$6,$7)")
            .bind(goals.id).bind(user.id()).bind(goals.protein_goal).bind(goals.carbs_goal)
            .bind(goals.fats_goal).bind(goals.tdee).bind(goals.bmr)
            .execute(&mut *transaction).await?;
        let saved_user = row.try_into()?;
        transaction.commit().await?;
        Ok(saved_user)
    }

    async fn get_user_by_id(&self, user_id: &Uuid) -> Result<Option<User>, RepositoryError> {
        sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE id=$1")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?
            .map(TryInto::try_into)
            .transpose()
    }
    async fn get_user_by_username(&self, username: &str) -> Result<Option<User>, RepositoryError> {
        sqlx::query_as::<_, UserRow>("SELECT * FROM users WHERE username=$1")
            .bind(username)
            .fetch_optional(&self.pool)
            .await?
            .map(TryInto::try_into)
            .transpose()
    }
    async fn get_sign_in_user_by_username(
        &self,
        username: &str,
    ) -> Result<Option<SignInUser>, RepositoryError> {
        Ok(sqlx::query_as::<_, SignInUserRow>(
            "SELECT id,username,password_hash FROM users WHERE username=$1",
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?
        .map(Into::into))
    }
    async fn exists(&self, user_id: &Uuid) -> Result<bool, RepositoryError> {
        Ok(
            sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE id=$1)")
                .bind(user_id)
                .fetch_one(&self.pool)
                .await?,
        )
    }
    async fn update_password(
        &self,
        user_id: &Uuid,
        new_password_hash: &str,
    ) -> Result<bool, RepositoryError> {
        Ok(sqlx::query("UPDATE users SET password_hash=$1 WHERE id=$2")
            .bind(new_password_hash)
            .bind(user_id)
            .execute(&self.pool)
            .await?
            .rows_affected()
            > 0)
    }
    async fn update_user(
        &self,
        user: &User,
        goals: &NutritionGoals,
    ) -> Result<User, RepositoryError> {
        let mut transaction = self.pool.begin().await?;
        let row = sqlx::query_as::<_, UserRow>(
            "UPDATE users SET weight=$1,height=$2,age=$3,activity_level=$4,goal=$5 WHERE id=$6 RETURNING *")
            .bind(user.profile().weight()).bind(user.profile().height()).bind(user.profile().age())
            .bind(ActivityLevelRow::from(user.profile().activity_level())).bind(GoalRow::from(user.profile().goal()))
            .bind(user.id()).fetch_one(&mut *transaction).await?;
        sqlx::query("INSERT INTO users_nutrition_goals (id,user_id,protein_goal,carbs_goal,fats_goal,tdee,bmr)
             VALUES ($1,$2,$3,$4,$5,$6,$7) ON CONFLICT(user_id) DO UPDATE SET
             protein_goal=EXCLUDED.protein_goal, carbs_goal=EXCLUDED.carbs_goal,
             fats_goal=EXCLUDED.fats_goal, tdee=EXCLUDED.tdee, bmr=EXCLUDED.bmr")
            .bind(goals.id).bind(user.id()).bind(goals.protein_goal).bind(goals.carbs_goal)
            .bind(goals.fats_goal).bind(goals.tdee).bind(goals.bmr)
            .execute(&mut *transaction).await?;
        let saved_user = row.try_into()?;
        transaction.commit().await?;
        Ok(saved_user)
    }
}
