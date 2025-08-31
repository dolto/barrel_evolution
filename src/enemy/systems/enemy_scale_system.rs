use bevy::prelude::*;

use crate::enemy::{structs::Enemy, systems::ENEMY_MAX_Z};

pub fn enemy_scale_system(mut enemys: Query<&mut Transform, With<Enemy>>) {
    for mut enemy_trans in enemys.iter_mut() {
        let enemy_pos = enemy_trans.translation;
        let scale = (1. - (enemy_pos.z / ENEMY_MAX_Z)) * 2.5 + 0.5;
        enemy_trans.scale = Vec3::splat(scale);
    }
}
