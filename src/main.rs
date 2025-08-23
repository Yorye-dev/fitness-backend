use std::io::{self, Write};
use dotenv::dotenv;
use std::env;

pub mod utils;
pub mod enums;
pub mod models;
pub mod congfig;

#[tokio::main]
async fn main() {

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Error en .env falta DATABASE_URL");

    println!("{}", database_url);

    let _pool = congfig::database::init_db_pg_pool(&database_url).await.unwrap();

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
