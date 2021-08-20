pub struct Damageable {
    pub health: i32,
}

impl Damageable {
    pub fn hurt(&mut self, damage: i32) {
        self.health -= damage;

        if self.health < 0 {
            self.health = 0;
        }
    }
}
