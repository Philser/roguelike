use std::collections::HashMap;

use bevy::prelude::*;

#[derive(Default)]
pub struct DamageTracker(pub HashMap<Entity, SufferDamage>);

pub struct SufferDamage {
    pub amount: Vec<i32>,
}

impl SufferDamage {
    pub fn add_damage(tracker: &mut ResMut<DamageTracker>, victim: Entity, amount: i32) {
        if let Some(damage_entry) = (*tracker).0.get_mut(&victim) {
            damage_entry.amount.push(amount);
        } else {
            tracker.0.insert(
                victim,
                SufferDamage {
                    amount: vec![amount],
                },
            );
        }
    }
}
