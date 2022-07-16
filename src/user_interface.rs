use bevy::prelude::*;

use crate::{
    components::CombatStats::CombatStats,
    map::RENDER_MAP_LABEL,
    player::{Player, PLAYER_STARTING_HEALTH},
    GameState,
};

pub struct UIPlugin {}

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system_set(
            SystemSet::on_update(GameState::Render)
                .with_system(render_ui)
                .before(RENDER_MAP_LABEL),
        );
    }
}

#[derive(Component)]
pub struct HealthBar {}

pub fn setup(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
    // root node
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            color: Color::PURPLE.into(),
            ..default()
        })
        .with_children(|parent| {
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
                .insert(HealthBar {});
        });
}

pub fn render_ui(
    player_query: Query<&CombatStats, With<Player>>,
    mut healthbar_query: Query<&mut Style, With<HealthBar>>,
) {
    let player = player_query
        .get_single()
        .expect("Got more or less than exactly one Player entity while rendering UI");

    let mut healthbar = healthbar_query
        .get_single_mut()
        .expect("Found more or less than exactly one Healthbar entity while rendering UI");
    healthbar.size.width = Val::Px(player.hp as f32 * 3.0);
}
