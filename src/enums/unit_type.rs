use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UnitType {
    Gram,
    Piece,
    Ml,
    Portion,
}
