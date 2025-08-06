use std::io::{self, Write};

pub mod utils;
pub mod enums;
pub mod models;

fn main() {

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

    
    println!("El tmp es: {}", utils::metrics::calculate_tdee(weight, height, age)) 
}
