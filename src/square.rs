use bevy::prelude::*;

pub const SQUARE_SIZE: f32 = 100.0;
pub struct SquarePlugin;

impl Plugin for SquarePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(square_state_system.system());
    }
}


#[derive(Clone, Debug, Copy)]
pub enum SquareState {
    Filled,
    Empty,
    _Crossed
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

pub fn square_state_system(
    mut materials: ResMut<Assets<ColorMaterial>>,
    entities: Query<&mut Handle<ColorMaterial>>,
    mut query: Query<(Entity, Changed<Square>)>) {
    for (entity, square) in &mut query.iter() {
        let mut sprite = entities.get_mut::<Handle<ColorMaterial>>(entity).unwrap();
        match square.current {
            SquareState::Empty => *sprite = materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
            SquareState::Filled => *sprite = materials.add(Color::rgb(0.1, 0.1, 0.9).into()),
            SquareState::_Crossed => *sprite = materials.add(Color::rgb(0.1, 0.1, 0.9).into()),
        }
    }
}
