use std::io::{self, Write};
use dotenv::dotenv;
use models::user::{User};
use std::env;
use uuid::Uuid;

pub mod utils;
pub mod enums;
pub mod models;
pub mod congfig;
pub mod repositories;

use repositories::user_repository::UserRepository;
use crate::enums::activity_level;

#[tokio::main]
async fn main() {

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Error en .env falta DATABASE_URL");

    println!("{}", database_url);

    let _pool = congfig::database::init_db_pg_pool(&database_url).await.unwrap();

    let user = User {
    id: Uuid::new_v4().to_string(),
    username: "jorge".into(),
    password_hash: "hashed_password".into(),
    age: 28,
    sex: "M".into(),
    height: 180,
    weight: 75.0,
    activity_level: activity_level::ActivityLevel::Sedentary.as_string(),
    };

    // Mapear de UserDto a User (ya con UUID y timestamp generados)
    //let user: User = dto.into();
    //
    let repo = UserRepository::new(_pool);

    match repo.create_user(&user).await {
        Ok(u) => println!("✅ Usuario creado: {:?}", u),
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    println!("Introduce la altura: ");
    io::stdout().flush().unwrap();
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Error al introducir la altura");
    let height: i32 = height.trim().parse().expect("Introduce un número válido");

    println!("Introduce la peso: ");
    io::stdout().flush().unwrap();
    let mut weight = String::new();
    io::stdin().read_line(&mut weight).expect("Error al introducir el peso");
    let weight: f32 = weight.trim().parse().expect("Introduce un número válido");

    println!("Introduce la edad: ");
    io::stdout().flush().unwrap();
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Error al introducir la edad");
    let age: i32 = age.trim().parse().expect("Introduce un número válido");

    // Conexión con la bdd
    
    //config
    println!("El tmp es: {}", utils::metrics::calculate_tdee(weight, height, age))
}
