use bevy::prelude::*;

pub struct UIPlugin {}

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}


pub fn setup(mut commands: Commands) {
    commands.spawn_bundle(
        NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            color: Color::PURPLE.into(),
            ..default()
        }
    );
}
