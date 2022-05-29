use bevy::prelude::Component;

#[derive(Component)]
pub struct Combattable {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

impl Combattable {
    pub fn hurt(&mut self, damage: i32) {
        self.hp -= damage;

        if self.hp < 0 {
            self.hp = 0;
        }
    }
}
