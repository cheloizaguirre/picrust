use bevy::prelude::*;

pub const SQUARE_SIZE: f32 = 100.0;
pub struct SquarePlugin;

impl Plugin for SquarePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<SquareMaterials>()
            .add_system(square_state_system.system());
    }
}

#[derive(Clone, Debug, Copy)]
pub enum SquareState {
    Filled,
    Empty,
    _Crossed,
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

struct SquareMaterials {
    empty: Handle<ColorMaterial>,
    filled: Handle<ColorMaterial>,
    crossed: Handle<ColorMaterial>,
}

impl FromResources for SquareMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        SquareMaterials {
            empty: materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
            filled: materials.add(Color::rgb(0.1, 0.1, 0.9).into()),
            crossed: materials.add(Color::rgb(0.1, 0.1, 0.9).into()),
        }
    }
}

fn square_state_system(
    materials: Res<SquareMaterials>,
    mut query: Query<(Changed<Square>, &mut Handle<ColorMaterial>)>,
) {
    for (square, mut material) in &mut query.iter() {
        match square.current {
            SquareState::Empty => *material = materials.empty,
            SquareState::Filled => *material = materials.filled,
            SquareState::_Crossed => *material = materials.crossed,
        }
    }
}
