use bevy::prelude::*;
mod square;
mod puzzle;
use crate::square::{SQUARE_SIZE};


fn startup_puzzle(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
    asset_server: Res<AssetServer>,
) {
    let (rows, cols) = (5, 5);
    let random_puzzle = puzzle::create_random_puzzle(rows, cols);
    let bounds = Vec2::new(SQUARE_SIZE * rows as f32, SQUARE_SIZE * cols as f32);
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default());
    let empty_texture_handle = asset_server
        .load_sync(&mut textures, "assets/empty_square.png")
        .unwrap();
    let square_size = Vec2::new(SQUARE_SIZE, SQUARE_SIZE);
    for (row_i, row) in random_puzzle.squares.rows_iter().enumerate() {
        for (col_i, _square) in row.enumerate() {
            let position = Vec3::new(
                col_i as f32 * SQUARE_SIZE - bounds.x() / 2.0,
                row_i as f32 * SQUARE_SIZE - bounds.y() / 2.0,
                0.0,
            );
            commands.spawn(SpriteComponents {
                material: materials.add(empty_texture_handle.into()),
                sprite: Sprite { size: square_size },
                scale: Scale(SQUARE_SIZE / 128.0),
                translation: Translation(position),
                ..Default::default()
            });
        }
    }
}

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(startup_puzzle.system())
        .run();
}
