use bevy::prelude::*;

use crate::{
    enemy::structs::Enemy,
    gun::gun::{Gun, GunControlStatus},
};

pub fn aim_circle_detecting_system(
    mut gun_status: ResMut<GunControlStatus>,
    gun_trans: Single<&GlobalTransform, With<Gun>>,
    enemys: Query<(&Transform, &Enemy)>,
) {
    let mut temp = false;
    for (enemy_trans, enemy) in enemys {
        let dist = (enemy_trans.translation - gun_trans.translation())
            .dot(gun_trans.rotation() * Vec3::NEG_Z)
            .abs();

        if dist <= enemy.size_deep * 0.7 {
            temp = true;
        }
    }

    gun_status.is_enemy_z = temp;
}
