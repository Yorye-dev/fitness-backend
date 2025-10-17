use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum UnitType {
    Gram,
    Piece,
    Ml,
    Portion,
}
