use crate::square::{Square, SquareState};
use array2d::Array2D;
use bevy::prelude::*;
use rand::Rng;

#[derive(Bundle)]
pub struct Puzzle {
    pub squares: Array2D<Square>,
}

impl Puzzle {
    pub fn rows(&self) -> usize {
        self.squares.num_rows()
    }
    pub fn cols(&self) -> usize {
        self.squares.num_columns()
    }
}

pub fn create_random_puzzle(rows: u32, columns: u32) -> Puzzle {
    let mut rng = rand::thread_rng();
    let random = || match rng.gen_range(0, 2) {
        x if x >= 1 => Square {
            solution: SquareState::Filled,
            ..Default::default()
        },
        _ => Square {
            ..Default::default()
        },
    };
    let squares = Array2D::filled_by_row_major(random, rows as usize, columns as usize);
    Puzzle { squares }
}
