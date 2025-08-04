use crate::utils::parsers;
use crate::enums::activity_level;

pub fn calculate_tdee(weight: f32, height:i32, age:i32) -> f32 {
    //Todo, con el usrio, registrado en bdd, como unico pará metro, tendría que funcionar
    //
    let tmb = calculate_bmr(weight, height, age);

    let activity = activity_level::ActivityLevel::VeryActive;

    tmb * activity.multiplier()
}

fn calculate_bmr(weight: f32, height:i32, age:i32) -> f32 {
    //Todo, con el usrio, registrado en bdd, como unico pará metro, tendría que funcionar
    10.00 * weight + 6.25 * parsers::convert_i32_at_f32(height) - 5.00 * parsers::convert_i32_at_f32(age) + 5.00
}


/*
 * fn calculate_BMR - implemetnar el calculo de TMB.
 * multipilcar ese tmb por el la actividad fisica
 * total de kcal al día. 
 *
 * luego calcular la cantidad de g de cada macro, en funcion de los objetivos.
 *
 *  casos especiales, superavit y déficit?
 *  
 *  mover esto a una capa de seVeryActive*
 *
 *
 */


