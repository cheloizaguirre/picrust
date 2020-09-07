use array2d::Array2D;
use bevy::prelude::*;
use rand::Rng;

#[derive(Clone, Debug, Copy)]
enum SquareState {
    Filled,
    Empty,
}

impl Default for SquareState {
    fn default() -> Self {
        SquareState::Empty
    }
}

fn create_random_puzzle(rows: u32, columns: u32) -> Puzzle {
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
    Puzzle(Array2D::filled_by_row_major(
        random,
        rows as usize,
        columns as usize,
    ))
}

fn startup_puzzle(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
    asset_server: Res<AssetServer>,
) {
    let (rows, cols) = (5, 5);
    let random_puzzle = create_random_puzzle(rows, cols);
    let puzzle_border_material = materials.add(Color::rgb(0.2, 0.2, 1.0).into());
    let border_thickness = 3.0;
    let bounds = Vec2::new(SQUARE_SIZE * rows as f32, SQUARE_SIZE * cols as f32);
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        // left
        .spawn(SpriteComponents {
            material: puzzle_border_material,
            translation: Translation(Vec3::new(-bounds.x() / 2.0, 0.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(border_thickness, bounds.y() + border_thickness),
            },
            ..Default::default()
        })
        // right
        .spawn(SpriteComponents {
            material: puzzle_border_material,
            translation: Translation(Vec3::new(bounds.x() / 2.0, 0.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(border_thickness, bounds.y() + border_thickness),
            },
            ..Default::default()
        })
        // bottom
        .spawn(SpriteComponents {
            material: puzzle_border_material,
            translation: Translation(Vec3::new(0.0, -bounds.y() / 2.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(bounds.x() + border_thickness, border_thickness),
            },
            ..Default::default()
        })
        // top
        .spawn(SpriteComponents {
            material: puzzle_border_material,
            translation: Translation(Vec3::new(0.0, bounds.y() / 2.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(bounds.x() + border_thickness, border_thickness),
            },
            ..Default::default()
        });

    let empty_texture_handle = asset_server.load_sync(&mut textures, "assets/empty_square.png").unwrap();
    let square_size = Vec2::new(SQUARE_SIZE, SQUARE_SIZE);
    for (row_i, row) in random_puzzle.0.rows_iter().enumerate() {
        for (col_i, _square) in row.enumerate() {
            let position = Vec3::new(col_i as f32 * SQUARE_SIZE - bounds.x() / 2.0, row_i as f32 * SQUARE_SIZE - bounds.y() / 2.0, 0.0);
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

struct Puzzle(Array2D<Square>);
const SQUARE_SIZE: f32 = 100.0;

#[derive(Debug, Clone, Copy)]
struct Square {
    solution: SquareState,
    current: SquareState,
    rel_pos: Vec2,
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

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(startup_puzzle.system())
        .run();
}
