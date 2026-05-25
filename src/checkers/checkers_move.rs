pub const BLACK_CORONATION_Y: usize = 7;
pub const RED_CORONATION_Y: usize = 0;

#[derive(Clone, Copy)]
pub struct Square {
    pub x: usize,
    pub y: usize
}

impl Square {

}

pub struct CheckersMove {
    pub start: Square,
    pub end: Square,
    pub capture: bool,
    pub coronation: bool
}