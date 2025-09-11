pub mod utils;
pub mod enums;
pub mod models;
pub mod congfig;
pub mod repositories;
pub mod dtos;
pub mod factories;

use std::io::{self, Write};
use dotenv::dotenv;
use models::user::{User};
use dtos::register_user_dto::RegisterUserDto;


use repositories::user_repository::UserRepository;
use crate::enums::activity_level;
use crate::factories::user_factory::UserFactory;

#[tokio::main]
async fn main() {

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Error en .env falta DATABASE_URL");

    println!("{}", database_url);

    let _pool = congfig::database::init_db_pg_pool(&database_url).await.unwrap();

    let register_user_dto = RegisterUserDto {
        username: "Paco".into(),
        plain_password: "123".into(),
        sex: "M".into(),
        weight: 83.0,
        height: 164,
        age: 24,
        activity_level: activity_level::ActivityLevel::Sedentary.as_string()
    };


    let user = UserFactory::create_user_from_dto(register_user_dto);

    // Mapear de UserDto a User (ya con UUID y timestamp generados)
    //let user: User = dto.into();
    //
    let repo = UserRepository::new(_pool.clone());

    match repo.save_user(&user).await {
        Ok(u) => println!("✅ Usuario creado: {:?}", u),
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    let repo = UserRepository::new(_pool.clone());
    
    match repo.get_public_user_by_username(&user.username).await {
        Ok(u) => println!("Este es el usuario: {:?}", u),
        Err(e) => println!("ERROR: {}", e),
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
