use crate::domain::{
    errors::RepositoryError,
    hydration::{DailyWater, WaterEntry, repository::HydrationRepository},
};
use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

pub struct SqlxHydrationRepository(PgPool);
impl SqlxHydrationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
}

async fn lock_owner(c: &mut PgConnection, user: Uuid) -> Result<(), RepositoryError> {
    sqlx::query("SELECT id FROM users WHERE id=$1 FOR UPDATE")
        .bind(user)
        .fetch_one(c)
        .await?;
    Ok(())
}
async fn read(
    c: &mut PgConnection,
    user: Uuid,
    date: NaiveDate,
) -> Result<DailyWater, RepositoryError> {
    let goal_ml: i32 = sqlx::query_scalar("SELECT COALESCE((SELECT goal_ml FROM water_goal_versions WHERE user_id=$1 AND effective_from<=$2 ORDER BY effective_from DESC LIMIT 1),2000)")
        .bind(user).bind(date).fetch_one(&mut *c).await?;
    let rows: Vec<(Uuid,i32)> = sqlx::query_as("SELECT id,amount_ml FROM water_intakes WHERE user_id=$1 AND local_date=$2 AND deleted_at IS NULL ORDER BY created_at,id")
        .bind(user).bind(date).fetch_all(&mut *c).await?;
    Ok(DailyWater {
        date,
        goal_ml,
        total_ml: rows.iter().map(|(_, n)| i64::from(*n)).sum(),
        entries: rows
            .into_iter()
            .map(|(id, amount_ml)| WaterEntry { id, amount_ml })
            .collect(),
    })
}
#[async_trait]
impl HydrationRepository for SqlxHydrationRepository {
    async fn daily(&self, user: Uuid, date: NaiveDate) -> Result<DailyWater, RepositoryError> {
        let mut tx = self.0.begin().await?;
        sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ, READ ONLY")
            .execute(&mut *tx)
            .await?;
        let day = read(&mut tx, user, date).await?;
        tx.commit().await?;
        Ok(day)
    }
    async fn add(
        &self,
        user: Uuid,
        date: NaiveDate,
        id: Uuid,
        amount: i32,
    ) -> Result<DailyWater, RepositoryError> {
        let mut tx = self.0.begin().await?;
        lock_owner(&mut tx, user).await?;
        let previous: Option<(Uuid,NaiveDate,i32,bool)>=sqlx::query_as("SELECT user_id,local_date,amount_ml,deleted_at IS NOT NULL FROM water_intakes WHERE id=$1")
            .bind(id).fetch_optional(&mut *tx).await?;
        if let Some((owner, day, ml, deleted)) = previous {
            if owner != user || day != date || ml != amount || deleted {
                return Err(RepositoryError::Conflict);
            }
        } else {
            let (total,count):(i64,i64)=sqlx::query_as("SELECT COALESCE(SUM(amount_ml),0)::bigint,COUNT(*) FROM water_intakes WHERE user_id=$1 AND local_date=$2 AND deleted_at IS NULL")
                .bind(user).bind(date).fetch_one(&mut *tx).await?;
            if total + i64::from(amount) > 50000 || count >= 1000 {
                return Err(RepositoryError::Conflict);
            }
            sqlx::query(
                "INSERT INTO water_intakes(id,user_id,local_date,amount_ml) VALUES($1,$2,$3,$4)",
            )
            .bind(id)
            .bind(user)
            .bind(date)
            .bind(amount)
            .execute(&mut *tx)
            .await?;
        }
        let day = read(&mut tx, user, date).await?;
        tx.commit().await?;
        Ok(day)
    }
    async fn remove(&self, user: Uuid, id: Uuid) -> Result<(), RepositoryError> {
        let mut tx = self.0.begin().await?;
        lock_owner(&mut tx, user).await?;
        let result=sqlx::query("UPDATE water_intakes SET deleted_at=COALESCE(deleted_at,CURRENT_TIMESTAMP) WHERE user_id=$1 AND id=$2")
            .bind(user).bind(id).execute(&mut *tx).await?;
        if result.rows_affected() == 0 {
            return Err(RepositoryError::NotFound);
        }
        tx.commit().await?;
        Ok(())
    }
    async fn set_goal(
        &self,
        user: Uuid,
        date: NaiveDate,
        goal: i32,
    ) -> Result<DailyWater, RepositoryError> {
        let mut tx = self.0.begin().await?;
        lock_owner(&mut tx, user).await?;
        sqlx::query("INSERT INTO water_goal_versions(user_id,effective_from,goal_ml) VALUES($1,$2,$3) ON CONFLICT(user_id,effective_from) DO UPDATE SET goal_ml=EXCLUDED.goal_ml")
            .bind(user).bind(date).bind(goal).execute(&mut *tx).await?;
        let day = read(&mut tx, user, date).await?;
        tx.commit().await?;
        Ok(day)
    }
}
