use bevy::prelude::*;

use crate::gun::gun::{Gun, GunAimCircle, GunControlStatus};
pub fn update_aim_circle_position(
    mut aim_trans: Single<&mut Transform, With<GunAimCircle>>,
    aim_pos: Res<GunControlStatus>,
) {
    aim_trans.translation = aim_pos.aim_position.extend(0.);
    aim_trans.translation.x += 5.;
}
