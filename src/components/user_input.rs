use bevy::prelude::Resource;

#[derive(Default, Resource, Debug)]
pub struct UserInput {
    pub x: i32,
    pub y: i32,
}
