use bevy::{
    prelude::{
        App, AssetServer, BuildChildren, Color, Commands, Component, DespawnRecursiveExt, Entity,
        GlobalTransform, Input, KeyCode, NodeBundle, Plugin, Query, Rect, Res, ResMut, Size, State,
        SystemSet, TextBundle, Transform, Vec3, With,
    },
    text::{Text, TextStyle},
    ui::{PositionType, Style, Val},
};

use crate::{configs::game_settings::GameConfig, GameState};

pub struct MainMenuPlugin {}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::MainMenu)
                .with_system(generate_main_menu)
                .label("main_menu_setup"),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::MainMenu)
                .with_system(main_menu_system)
                .after("main_menu_setup"),
        );
    }
}

#[derive(Component)]
pub struct MainMenuUI {}

fn generate_main_menu(
    mut commands: Commands,
    game_config: Res<GameConfig>,
    asset_server: Res<AssetServer>,
) {
    let menu_points = vec!["New Game", "Save", "Load", "Quit"];
    let text_font = asset_server.load("fonts/EduVICWANTBeginner-Regular.ttf");
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(
                    Val::Px(game_config.screen_dimensions.screen_width),
                    Val::Px(game_config.screen_dimensions.screen_height),
                ),
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            color: Color::rgba(0.0, 0.0, 0.0, 0.7).into(),
            ..Default::default()
        })
        .insert(MainMenuUI {})
        .with_children(|parent| {
            for (i, point) in menu_points.iter().enumerate() {
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(33.3), Val::Percent(10.0)),
                            position_type: PositionType::Absolute,
                            position: Rect {
                                bottom: Val::Percent((menu_points.len() - i) as f32 * 20.0),
                                left: Val::Percent(33.3),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        color: Color::PINK.into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text::with_section(
                                *point,
                                TextStyle {
                                    font: text_font.clone(),
                                    font_size: 27.0,
                                    color: Color::WHITE,
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        });
                    });
            }
        });
}

fn main_menu_system(
    mut app_state: ResMut<State<GameState>>,
    mut commands: Commands,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    main_menu_ui: Query<Entity, With<MainMenuUI>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        let main_menu = main_menu_ui
            .get_single()
            .expect("Fetching the main menu UI");
        commands.entity(main_menu).despawn_recursive();

        app_state.pop().expect("Popping main menu game state");
    }
    keyboard_input.clear();
}
