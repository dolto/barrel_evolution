use bevy::prelude::*;

use crate::gun::gun::{Gun, GunControlStatus};

pub fn gun_cooling_system(
    mut gun: Single<&mut Gun>,
    time: Res<Time>,
    gun_status: Res<GunControlStatus>,
) {
    if gun_status.firing && gun_status.aiming {
        return;
    }
    let heal = gun.heal;
    for barrel in gun.barrels.iter_mut() {
        barrel.hp = (barrel.hp - heal * time.delta_secs()).max(0.);

        if barrel.is_broken && barrel.hp == 0. {
            barrel.is_broken = false;
        }
    }
}
