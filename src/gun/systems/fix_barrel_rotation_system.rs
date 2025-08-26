use bevy::prelude::*;

use crate::gun::{barrel::BarrelSprite, gun::Gun};

pub fn fix_barrel_rotation_system(
    time: Res<Time>,
    gun: Single<&Gun>,
    mut barrel_query: Query<&mut Transform, With<BarrelSprite>>,
) {
    for mut b_trans in barrel_query.iter_mut() {
        b_trans.rotate_y(-gun.speed * time.delta_secs());
    }
}
