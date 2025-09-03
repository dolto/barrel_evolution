use bevy::prelude::*;

use crate::{effect::structs::Effect, enemy::systems::ENEMY_MAX_Z};

pub fn effect_scale_system(mut enemys: Query<&mut Transform, With<Effect>>) {
    for mut enemy_trans in enemys.iter_mut() {
        let enemy_pos = enemy_trans.translation;
        let scale = (1. - (enemy_pos.z / ENEMY_MAX_Z)) * 2.5 + 0.5;
        enemy_trans.scale = Vec3::splat(scale);
    }
}
