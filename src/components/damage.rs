use bevy::prelude::Component;

#[derive(Component, Debug)]
pub struct InflictsDamage {
    pub damage: i32,
}

use std::collections::HashMap;

use bevy::prelude::*;

use crate::user_interface::ActionLog;

#[derive(Default)]
pub struct DamageTracker(pub HashMap<Entity, SufferDamage>);

pub struct SufferDamage {
    pub amount: Vec<i32>,
}

impl SufferDamage {
    pub fn add_damage(
        tracker: &mut ResMut<DamageTracker>,
        victim: Entity,
        amount: i32,
        action_log: &mut ActionLog,
        attacker_is_player: bool,
    ) {
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

        let text = match attacker_is_player {
            true => format!("Player hits Monster for {}", amount),
            false => format!("Monster hits Player for {}", amount),
        };
        action_log.entries.push(text);
    }
}
