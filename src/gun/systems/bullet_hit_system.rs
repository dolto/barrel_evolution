use bevy::prelude::*;

use crate::{
    enemy::{structs::Enemy, systems::ENEMY_MAX_Z},
    gun::{barrel::Bullet, systems::DESPAWN_BULLETS_Z},
};

pub fn bullet_hit_system(
    mut commands: Commands,
    bullet_query: Query<(&GlobalTransform, &Bullet, Entity)>,
    mut enemy_query: Query<(&GlobalTransform, &mut Enemy, Entity)>,
) {
    for (b_trans, bullet, b_entity) in bullet_query {
        let b_pos = b_trans.translation();
        let bz = b_pos.z;
        for (e_trans, mut enemy, e_entity) in enemy_query.iter_mut() {
            let e_pos = e_trans.translation();
            let e_scale = (1. - (e_pos.z / ENEMY_MAX_Z)) * 1.5 + 0.5;
            let b_scale = (DESPAWN_BULLETS_Z - b_pos.z) / DESPAWN_BULLETS_Z;
            let enemy_side = enemy.size_side * e_scale;
            let bullet_side = bullet.size * b_scale;
            if e_pos.with_z(bz).distance(b_pos) >= enemy_side + bullet_side
                || (e_pos.z - b_pos.z).abs() >= enemy.size_deep + bullet.size
            {
                continue;
            }

            enemy.hp -= bullet.damage;
            bullet.hit(&mut commands, b_entity);
            if enemy.hp <= 0. {
                enemy.dead(&mut commands, e_entity);
            }
        }
    }
}
