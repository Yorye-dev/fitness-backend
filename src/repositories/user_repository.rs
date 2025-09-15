use sqlx::{Pool, Postgres};

use crate::models::user::{User, PublicUser};

const USER_TABLE: &str = "users";

#[derive(Clone)]
pub struct UserRepository {
    pool: Pool<Postgres>,
}

impl UserRepository  {
    
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn save_user(&self, user: &User) -> Result<User, sqlx::Error> {

        let query = format!("
            INSERT INTO {} (id, username, password_hash, age, sex, height, weight, activity_level)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, username, password_hash, age, sex, height, weight, activity_level
            ", USER_TABLE);

        let saved_user = sqlx::query_as::<_, User>(&query)
        .bind(&user.id)
        .bind(&user.username)
        .bind(&user.password_hash)
        .bind(user.age)
        .bind(&user.sex)
        .bind(user.height)
        .bind(user.weight)
        .bind(&user.activity_level)
        .fetch_one(&self.pool)
        .await?;

        Ok(saved_user)
    }

    pub async fn get_public_user_by_username(&self, username: &String) -> Result<PublicUser, sqlx::Error> {
        
        let query = format!("SELECT id, username, age, sex, height, weight, activity_level 
            FROM {} 
            WHERE username = $1",USER_TABLE);

        let public_user = sqlx::query_as::<_, PublicUser>(&query)
            .bind(username)
            .fetch_one(&self.pool)
            .await?;

        Ok(public_user)
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
