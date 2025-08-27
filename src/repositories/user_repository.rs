use sqlx::{Pool, Postgres};

use crate::models::user::User;

pub struct UserRepository {
    pool: Pool<Postgres>,
}

impl UserRepository  {
    
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, user: &User) -> Result<User, sqlx::Error> {
        let row = sqlx::query_as!(
            User,
            r#"
            INSERT INTO "00_user" (id, username, password_hash, age, sex, height, weight, activity_level)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, username, password_hash, age, sex, height, weight, activity_level
            "#,
            user.id,
            user.username,
            user.password_hash,
            user.age,
            user.sex,
            user.height,
            user.weight,
            user.activity_level
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row)
    }
}

//impl<'a> UserRepository<'a> {

  //  pub async fn create_user(&self, user: &User) -> Result<User, sqlx::Error> {
   //     let row = sqlx::query_as!(
    //        User,
     //       r#"
      //      INSERT INTO 00_user (id, username, email, password_hash, age, sex, height, weight, activity_level)
       //     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        //    RETURNING id, username, password_hash, age, sex, height, weight, activity_level, created_at
         //   "#,
         //   user.id,
          //  user.username,
          //  user.email,
          //  user.password_hash,
           // user.age,

       //     user.sex,
       //     user.height,
       //     user.weight,
        //    user.activity_level,
        //    user.created_at
      //  )
      //  .fetch_one(&self.pool)
      //  .await?;

      //  Ok(row)
    //}
            //}//
