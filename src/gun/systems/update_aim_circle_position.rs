use bevy::prelude::*;

use crate::gun::gun::{GunAimCircle, GunControlStatus};
pub fn update_aim_circle_position(
    mut aim_trans: Single<&mut Transform, With<GunAimCircle>>,
    aim_pos: Res<GunControlStatus>,
) {
    aim_trans.translation = aim_pos.aim_position.extend(0.);
    aim_trans.translation.x += 5.;

    let scale = ((aim_pos.aim_position.y + 200.) / 450.) * 1.5 + 0.5;
    aim_trans.scale = Vec3::splat(scale);
}
