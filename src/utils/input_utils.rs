use bevy::prelude::{Input, KeyCode};

pub struct MovementInput {
    pub x: i32,
    pub y: i32,
}

impl MovementInput {
    pub fn received_movement_input(&self) -> bool {
        return self.x != 0 || self.y != 0;
    }
}

pub fn get_movement_input(keyboard_input: &Input<KeyCode>) -> MovementInput {
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

    MovementInput { x, y }
}
