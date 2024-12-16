use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CoordsArea {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}
