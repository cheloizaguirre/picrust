use bevy::{prelude::*, render::pass::ClearColor};
mod puzzle;
mod square;
mod ui_elements;
use crate::puzzle::Puzzle;
use crate::square::{SQUARE_SIZE, SquarePlugin};
use crate::ui_elements::{UIPlugin, PuzzleCanvasMarker};

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default());
    let puzzle = puzzle::create_random_puzzle(5,5);
        let (rows, cols) = (puzzle.rows(), puzzle.cols());

        let bounds = Vec2::new(SQUARE_SIZE * rows as f32, SQUARE_SIZE * cols as f32);
        let square_size = Vec2::new(SQUARE_SIZE, SQUARE_SIZE);
        for (row_i, row) in puzzle.squares.rows_iter().enumerate() {
            for (col_i, square) in row.enumerate() {
                let position = Vec3::new(
                    col_i as f32 * SQUARE_SIZE - bounds.x() / 2.0,
                    row_i as f32 * SQUARE_SIZE - bounds.y() / 2.0,
                    1.0,
                );
                commands
                    .spawn(SpriteComponents {
                        sprite: Sprite { size: square_size },
                        scale: Scale(SQUARE_SIZE / 128.0),
                        translation: Translation(position),
                        ..Default::default()
                    })
                .with(square.clone());
                }
        }
}

fn puzzle_system(mut commands: Commands, 
    puzzle: &Puzzle
){
    //for puzzle in &mut puzzle_query.iter() {
        let (rows, cols) = (puzzle.rows(), puzzle.cols());

        let bounds = Vec2::new(SQUARE_SIZE * rows as f32, SQUARE_SIZE * cols as f32);
        let square_size = Vec2::new(SQUARE_SIZE, SQUARE_SIZE);
        for (row_i, row) in puzzle.squares.rows_iter().enumerate() {
            for (col_i, square) in row.enumerate() {
                let position = Vec3::new(
                    col_i as f32 * SQUARE_SIZE - bounds.x() / 2.0,
                    row_i as f32 * SQUARE_SIZE - bounds.y() / 2.0,
                    1.0,
                );
                commands
                    .spawn(SpriteComponents {
                        sprite: Sprite { size: square_size },
                        scale: Scale(SQUARE_SIZE / 128.0),
                        translation: Translation(position),
                        ..Default::default()
                    })
                .with(square.clone());
                }
        }
    //}
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Picrust".to_string(),
            width: 1920,
            height: 1080,
            vsync: true,
            resizable: true,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::BLACK))
        .add_default_plugins()
        .add_plugin(UIPlugin)
        .add_plugin(SquarePlugin)
        .add_startup_system(setup.system())
        .add_system(puzzle_system.system())
        .run();
}
