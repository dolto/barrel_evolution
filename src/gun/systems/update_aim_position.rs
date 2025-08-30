use bevy::prelude::*;

use crate::gun::gun::{Gun, GunControlStatus};

pub fn update_aim_position(
    window: Single<&Window>,
    touches: Res<Touches>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut gun_control_status: ResMut<GunControlStatus>,
    gun: Single<&Gun>,
    time: Res<Time>,
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
            let dir = (pos - gun_control_status.aim_position).normalize();
            let mut g_pos =
                gun_control_status.aim_position + dir * time.delta_secs() * gun.aim_speed;

            g_pos.y = g_pos.y.clamp(-200.0, 250.0);
            gun_control_status.aim_position = g_pos;
        }
    }
}
