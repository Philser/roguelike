use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::{
    components::CombatStats::CombatStats,
    map::RENDER_MAP_LABEL,
    player::{Player, PLAYER_STARTING_HEALTH},
    GameState,
};

pub struct UIPlugin {}

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system_set(
                SystemSet::on_update(GameState::Render)
                    .with_system(render_ui)
                    .after(RENDER_MAP_LABEL),
            )
            .add_system_set(SystemSet::on_enter(GameState::AwaitingInput).with_system(render_ui));
    }
}

#[derive(Component)]
pub struct HealthBar {}

#[derive(Component)]
pub struct HealthText {}

#[derive(Component)]
pub struct CombatLog {}

pub struct UIFont(Handle<Font>);

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    let font_handle: Handle<Font> = asset_server.load("fonts\\EduVICWANTBeginner-Regular.ttf");
    commands.insert_resource(UIFont(font_handle.clone()));

    let mut commands_builder = commands.spawn_bundle(NodeBundle {
        style: Style {
            justify_content: JustifyContent::SpaceBetween,
            size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
            ..default()
        },
        color: Color::PURPLE.into(),
        ..default()
    });
    spawn_health_bar(&mut commands_builder, font_handle.clone());
    spawn_combat_log(&mut commands_builder, font_handle);
}

pub fn spawn_health_bar(commands: &mut EntityCommands, text_font: Handle<Font>) {
    commands.with_children(|parent| {
        parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(
                        Val::Px(PLAYER_STARTING_HEALTH as f32 * 3.0),
                        Val::Percent(10.0),
                    ),
                    position_type: PositionType::Absolute,
                    position: Rect {
                        left: Val::Percent(50.0),
                        top: Val::Percent(25.0),
                        ..default()
                    },
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                color: Color::RED.into(),
                ..default()
            })
            .insert(HealthBar {})
            .with_children(|parent| {
                parent
                    .spawn_bundle(TextBundle {
                        style: Style {
                            margin: Rect::all(Val::Px(5.0)),
                            ..default()
                        },
                        text: Text::with_section(
                            format!("{}/{}", PLAYER_STARTING_HEALTH, PLAYER_STARTING_HEALTH),
                            TextStyle {
                                font: text_font,
                                font_size: 27.0,
                                color: Color::WHITE,
                            },
                            Default::default(),
                        ),
                        ..default()
                    })
                    .insert(HealthText {});
            });
    });
}

pub fn spawn_combat_log(commands: &mut EntityCommands, text_font: Handle<Font>) {
    commands.with_children(|parent| {
        parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::ColumnReverse,
                    size: Size::new(Val::Percent(40.0), Val::Percent(100.0)),
                    ..default()
                },
                color: Color::rgb(0.8, 0.8, 1.0).into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect {
                            left: Val::Percent(1.0),
                            right: Val::Percent(1.0),
                            bottom: Val::Percent(1.0),
                            ..default()
                        },
                        ..default()
                    },
                    text: Text::with_section(
                        "Adventure awaits!",
                        TextStyle {
                            font: text_font,
                            font_size: 27.0,
                            color: Color::WHITE,
                        },
                        Default::default(),
                    ),
                    ..default()
                });
            })
            .insert(CombatLog {});
    });
}

pub fn render_ui(
    player_query: Query<&CombatStats, With<Player>>,
    mut healthbar_query: Query<&mut Style, With<HealthBar>>,
    mut healthtext_query: Query<&mut Text, With<HealthText>>,
) {
    let player = player_query
        .get_single()
        .expect("Got more or less than exactly one Player entity while rendering UI");

    let mut healthbar = healthbar_query
        .get_single_mut()
        .expect("Found more or less than exactly one Healthbar entity while rendering UI");

    healthbar.size.width = Val::Px(player.hp as f32 * 3.0);

    let mut healthtext = healthtext_query
        .get_single_mut()
        .expect("Found more or less than exactly one Healthtext entity while rendering UI");

    // We only care for the first section
    healthtext.sections[0].value = format!("{}/{}", player.hp, player.max_hp);
}
