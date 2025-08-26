use bevy::prelude::*;

use crate::gun::gun::GunControlStatus;

pub fn update_aim_position(
    window: Single<&Window>,
    touches: Res<Touches>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut gun_control_status: ResMut<GunControlStatus>,
) {
    if !gun_control_status.aiming {
        return;
    }
    let (camera, camera_trans) = camera.into_inner();
    let mut aim_positon = Vec2::ZERO;
    let mut is_pos = false;
    if let Some(pos) = window.cursor_position() {
        aim_positon = pos;
        is_pos = true;
    }
    for touch in touches.iter() {
        aim_positon = touch.position();
        is_pos = true;
    }

    let aim = camera.viewport_to_world_2d(camera_trans, aim_positon);

    if let Ok(pos) = aim {
        if is_pos {
            gun_control_status.aim_position = pos;
        }
    }
}
