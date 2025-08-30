use bevy::prelude::*;

use crate::gun::gun::{Gun, GunSpin};

pub fn rotate_gun_system(
    gun: Single<&Gun>,
    time: Res<Time>,
    gun_spin: Single<&mut Transform, With<GunSpin>>,
) {
    gun_spin
        .into_inner()
        .rotate_y(gun.speed * time.delta_secs());
} 
