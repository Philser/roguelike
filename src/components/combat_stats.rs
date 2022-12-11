use bevy::prelude::Component;

#[derive(Component)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

impl CombatStats {
    pub fn hurt(&mut self, damage: i32) {
        self.hp -= damage;

        if self.hp < 0 {
            self.hp = 0;
        }
    }

    pub fn heal(&mut self, heal: i32) {
        self.hp += heal;

        if self.hp > self.max_hp {
            self.hp = self.max_hp;
        }
    }
}
