use serde::{Deserialize, Serialize};
use xcap::Window;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}

impl Coords {
    pub fn relative_to_window(&self, window: &Window) -> Coords {
        Coords {
            x: window.x() + self.x,
            y: window.y() + self.y,
        }
    }

    pub fn relative_to_origin(&self, origin: &Coords) -> Coords {
        Coords {
            x: origin.x + self.x,
            y: origin.y + self.y,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CoordsArea {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl CoordsArea {
    pub fn relative_to_window(&self, window: &Window) -> CoordsArea {
        CoordsArea {
            x: window.x() + self.x,
            y: window.y() + self.y,
            w: self.w,
            h: self.h,
        }
    }

    pub fn relative_to_origin(&self, origin: &Coords) -> CoordsArea {
        CoordsArea {
            x: origin.x + self.x,
            y: origin.y + self.y,
            w: self.w,
            h: self.h,
        }
    }

    pub fn contains(&self, point: &Coords) -> bool {
        point.x >= self.x && point.x < self.x + self.w && point.y >= self.y && point.y < self.y + self.h
    }
}
