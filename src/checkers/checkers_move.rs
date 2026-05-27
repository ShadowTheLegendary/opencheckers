pub const BLACK_CORONATION_Y: u64 = 7;
pub const RED_CORONATION_Y: u64 = 0;

#[derive(Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Square {
    pub x: u64,
    pub y: u64
}

impl Square {
    pub fn new() -> Self {
        Square { x: 0, y: 0 }
    }

    pub fn apply(&self, offset: &(i32, i32)) -> Self {
        Square {
            x: (self.x as i32 + offset.0) as u64,
            y: (self.y as i32 + offset.1) as u64
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CheckersMove {
    pub start: Square,
    pub end: Square,
    pub capture: bool,
    pub coronation: bool
}

impl CheckersMove {
    pub fn new() -> Self {
        CheckersMove { start: Square::new(), end: Square::new(), capture: false, coronation: false }
    }
}