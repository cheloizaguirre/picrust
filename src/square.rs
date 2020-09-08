use bevy::prelude::*;

pub const SQUARE_SIZE: f32 = 100.0;
pub struct SquarePlugin;

impl Plugin for SquarePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SquareMaterials>()
            .add_system(square_state_system.system());
    }
}


#[derive(Clone, Debug, Copy)]
pub enum SquareState {
    Filled,
    Empty,
    Crossed
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

pub struct SquareMaterials {
    empty: Handle<Texture>,
    filled: Handle<Texture>,
    crossed: Handle<Texture>
}

impl FromResources for SquareMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let asset_server = resources.get::<AssetServer>().unwrap();
        let mut textures = resources.get_mut::<Assets<Texture>>().unwrap();
        let empty_texture_handle = asset_server
            .load_sync(&mut textures, "assets/empty_square.png")
            .unwrap();
        let filled_texture_handle = asset_server
            .load_sync(&mut textures, "assets/filled_square.png")
            .unwrap();
        SquareMaterials {
            empty: empty_texture_handle.into(),
            filled: filled_texture_handle.into(),
            crossed: empty_texture_handle.into(),
        }
    }
}

pub fn square_state_system(
    materials: Res<SquareMaterials>,
    mut query: Query<(&mut Handle<Texture>, &mut Square)>) {
    for (mut texture, square) in &mut query.iter() {
        match square.current {
            SquareState::Filled => *texture = materials.filled,
            SquareState::Empty => *texture = materials.empty,
            SquareState::Crossed => *texture = materials.crossed,
        }
    }
}
