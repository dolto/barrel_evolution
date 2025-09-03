use bevy::prelude::*;

use crate::{
    effect::structs::EffectMaker,
    enemy::{structs::Enemy, systems::ENEMY_MAX_Z},
    gun::{barrel::Bullet, systems::DESPAWN_BULLETS_Z},
    util::DeadFlag,
};

pub fn bullet_hit_system(
    bullet_query: Query<
        (
            &GlobalTransform,
            &Bullet,
            &mut DeadFlag,
            Option<&mut EffectMaker>,
        ),
        With<Bullet>,
    >,
    mut enemy_query: Query<
        (
            &GlobalTransform,
            &mut Enemy,
            &mut DeadFlag,
            Option<&mut EffectMaker>,
        ),
        (With<Enemy>, Without<Bullet>),
    >,
) {
    for (b_trans, bullet, mut b_dead_flag, mut b_effect_maker) in bullet_query {
        let b_pos = b_trans.translation();
        let bz = b_pos.z;
        for (e_trans, mut enemy, mut e_dead_flag, mut e_effect_maker) in enemy_query.iter_mut() {
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
            b_dead_flag.0 = true;

            if let Some(ref mut b_effect_maker) = b_effect_maker {
                b_effect_maker.make_flag = true;
            }
            if let Some(ref mut e_effect_maker) = e_effect_maker {
                e_effect_maker.make_flag = true;
            }
            if enemy.hp <= 0. {
                enemy.dead(&mut e_dead_flag);
            }
        }
    }
}
