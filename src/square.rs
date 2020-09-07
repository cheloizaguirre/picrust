use bevy::prelude::Vec2;

pub const SQUARE_SIZE: f32 = 100.0;

#[derive(Clone, Debug, Copy)]
pub enum SquareState {
    Filled,
    Empty,
}

impl Default for SquareState {
    fn default() -> Self {
        SquareState::Empty
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Square {
    pub solution: SquareState,
    pub current: SquareState,
    pub rel_pos: Vec2,
}
impl Default for Square {
    fn default() -> Self {
        Square {
            solution: SquareState::Empty,
            current: SquareState::Empty,
            rel_pos: Vec2::new(0.0, 0.0),
        }
    }
}
