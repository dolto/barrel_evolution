use bevy::prelude::*;

use crate::{
    enemy::{aim::Aim, structs::Enemy},
    gun::gun::Gun,
};

pub fn aim_circle_detecting_system(
    gun_trans: Single<&GlobalTransform, With<Gun>>,
    enemys: Query<(&Transform, &Enemy, &Children)>,
    mut aims: Query<&mut Visibility, With<Aim>>,
) {
    for (enemy_trans, enemy, child) in enemys {
        let dist = (enemy_trans.translation - gun_trans.translation())
            .dot(gun_trans.rotation() * Vec3::NEG_Z)
            .abs();

        let mut aim = None;
        for child_entity in child {
            aim = Some(aims.get_mut(*child_entity).unwrap());
        }
        if let Some(mut aim) = aim {
            if dist <= enemy.size_deep * 0.7 {
                *aim = Visibility::Visible;
            } else {
                *aim = Visibility::Hidden;
            }
        }
    }
}
