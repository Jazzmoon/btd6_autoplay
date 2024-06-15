use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}

impl Coords {
    fn new(x: i32, y: i32) -> Coords {
        Coords { x, y }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoordsArea {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl CoordsArea {
    fn new(x: i32, y: i32, w: i32, h: i32) -> CoordsArea {
        CoordsArea { x, y, w, h }
    }
}