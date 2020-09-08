use bevy::prelude::*;
use crate::puzzle;
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<UIMaterials>()
            .add_startup_system(setup_ui.system())
            .add_system(ui_system.system());
    }
}

pub struct PuzzleCanvasMarker {}
pub struct UICanvasMarker {}

struct UIMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
    area_background: Handle<ColorMaterial>,
}

impl FromResources for UIMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        UIMaterials {
            normal: materials.add(Color::rgb(0.02, 0.02, 0.02).into()),
            hovered: materials.add(Color::rgb(0.05, 0.05, 0.05).into()),
            pressed: materials.add(Color::rgb(0.1, 0.5, 0.1).into()),
            area_background: materials.add(Color::BLACK.into()),
        }
    }
}

fn ui_system(
    mut commands: Commands,
    button_materials: Res<UIMaterials>,
    mut interaction_query: Query<(
        &Button,
        Mutated<Interaction>,
        &mut Handle<ColorMaterial>,
        &Children,
    )>,
) {
    for (_button, interaction, mut material, _children) in &mut interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed;
                commands.spawn(puzzle::create_random_puzzle(5, 5));
            }
            Interaction::Hovered => {
                *material = button_materials.hovered;
            }
            Interaction::None => {
                *material = button_materials.normal;
            }
        }
    }
}
fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<UIMaterials>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            material: color_materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            // Puzzle canvas
            let puzzle_canvas = NodeComponents {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(90.0)),
                        position_type: PositionType::Absolute,
                        position: Rect {
                            left: Val::Px(0.0),
                            top: Val::Px(0.0),
                            ..Default::default()
                        },
                        border: Rect::all(Val::Px(2.0)),
                        ..Default::default()
                    },
                    material: color_materials.add(Color::BLACK.into()),
                    ..Default::default()
                };
            parent.spawn(puzzle_canvas).with(PuzzleCanvasMarker{});
            parent
                .spawn(NodeComponents {
                    style: Style {
                        size: Size::new(Val::Percent(50.0), Val::Percent(10.0)),
                        position_type: PositionType::Relative,
                        position: Rect {
                            left: Val::Px(0.0),
                            top: Val::Px(0.0),
                            ..Default::default()
                        },
                        align_items: AlignItems::FlexStart,
                        border: Rect::all(Val::Px(2.0)),
                        ..Default::default()
                    },
                    material: button_materials.area_background,
                    ..Default::default()
                })
                .with(UICanvasMarker {})
                .with_children(|ui_canvas| {
                    ui_canvas
                        .spawn(ButtonComponents {
                            style: Style {
                                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            material: button_materials.normal,
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextComponents {
                                text: Text {
                                    value: "New".to_string(),
                                    font: asset_server
                                        .load(
                                            "assets/fonts/Meslo LG S DZ Regular for Powerline.ttf",
                                        )
                                        .unwrap(),
                                    style: TextStyle {
                                        font_size: 40.0,
                                        color: Color::rgb(0.8, 0.8, 0.8),
                                    },
                                },
                                ..Default::default()
                            });
                        })
                        .spawn(ButtonComponents {
                            style: Style {
                                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            material: button_materials.normal,
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextComponents {
                                text: Text {
                                    value: "Solve".to_string(),
                                    font: asset_server
                                        .load(
                                            "assets/fonts/Meslo LG S DZ Regular for Powerline.ttf",
                                        )
                                        .unwrap(),
                                    style: TextStyle {
                                        font_size: 40.0,
                                        color: Color::rgb(0.8, 0.8, 0.8),
                                    },
                                },
                                ..Default::default()
                            });
                        });
                });
        });
}
