struct Coords {
    x: i32,
    y: i32,
}

impl Coords {
    fn new(x: i32, y: i32) -> Coords {
        Coords { x, y }
    }
}

struct CoordsArea {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl CoordsArea {
    fn new(x: i32, y: i32, w: i32, h: i32) -> CoordsArea {
        CoordsArea { x, y, w, h }
    }
}