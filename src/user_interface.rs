use bevy::prelude::*;

use crate::{player::PLAYER_STARTING_HEALTH, SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct UIPlugin {}

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

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
            parent.spawn_bundle(NodeBundle {
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
            });
        });
}
