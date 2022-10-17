use bevy::prelude::{Input, KeyCode, ResMut};

pub struct WASDMovement {
    pub x: i32,
    pub y: i32,
}

impl WASDMovement {
    pub fn received_input(&self) -> bool {
        return self.x != 0 || self.y != 0;
    }
}

pub fn get_WASD_movement(keyboard_input: &ResMut<Input<KeyCode>>) -> WASDMovement {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    if keyboard_input.just_pressed(KeyCode::A) {
        x = -1;
    }
    if keyboard_input.just_pressed(KeyCode::D) {
        x = 1;
    }
    if keyboard_input.just_pressed(KeyCode::W) {
        y = 1;
    }
    if keyboard_input.just_pressed(KeyCode::S) {
        y = -1;
    }

    WASDMovement { x, y }
}
