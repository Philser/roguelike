use bevy::{
    prelude::{
        App, Color, Commands, Component, Entity, GlobalTransform, Input, KeyCode, NodeBundle,
        Plugin, Query, Rect, Res, ResMut, Size, State, SystemSet, Transform, Vec3, With,
    },
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

fn generate_main_menu(mut commands: Commands, game_config: Res<GameConfig>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(
                    Val::Px(game_config.screen_dimensions.screen_width),
                    Val::Px(game_config.screen_dimensions.screen_height),
                ),
                position_type: PositionType::Absolute,
                // position: Rect {
                //     left: Val::Px(0.0),
                //     top: Val::Px(0.0),
                //     right: Val::Px(game_config.screen_dimensions.screen_width),
                //     bottom: Val::Px(game_config.screen_dimensions.screen_height),
                // },
                ..Default::default()
            },
            global_transform: GlobalTransform {
                translation: Vec3::new(0.0, 0.0, 100.0),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 100.0),
                ..Default::default()
            },
            color: Color::rgba(0.0, 0.0, 0.0, 0.7).into(),
            ..Default::default()
        })
        .insert(MainMenuUI {});
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
        commands.entity(main_menu).despawn();

        app_state.pop().expect("Popping main menu game state");
    }
    keyboard_input.clear();
}
