use bevy::prelude::*;

use crate::gun::gun::{Gun, GunControlStatus};

pub fn rotate_to_gun_system(
    time: Res<Time>,
    window: Single<&Window>,
    gun: Single<(&Gun, &mut Transform)>,
    aim: Res<GunControlStatus>,
) {
    let (gun, mut g_trans) = gun.into_inner();
    let g_tal = g_trans.translation;
    let aim_pos = aim.aim_position;
    let aim_speed = gun.aim_speed;

    // --- Z축 회전 (기존) ---
    let gun_forward = (g_trans.rotation * Vec3::Y).xy();
    let to_aim = (aim_pos - g_tal.xy()).normalize();
    let forward_dot_aim = gun_forward.dot(to_aim);
    let gun_right = (g_trans.rotation * Vec3::X).xy();
    let right_dot_aim = gun_right.dot(to_aim);
    let rotation_sign = -f32::copysign(1.0, right_dot_aim);
    let max_angle = f32::acos(forward_dot_aim.clamp(-1.0, 1.0));
    let rotation_angle = rotation_sign * (aim_speed * time.delta_secs()).min(max_angle);
    g_trans.rotate_z(rotation_angle);

    // --- X축 회전 (속도 제한 적용) ---
    let window = window.into_inner();
    let win_height = window.height() / 2.;
    let screen_y = win_height - aim_pos.y.clamp(-win_height, win_height);
    let target_x_deg = (screen_y / window.height()) * 65.0;
    let target_x_rad = target_x_deg.to_radians();

    // 현재 X축 회전
    let (current_x, _, current_z) = g_trans.rotation.to_euler(EulerRot::XYZ);

    // 목표 각도와 현재 각도 차이
    let delta = target_x_rad - current_x;

    // 제한된 회전량 (aim_speed에 맞춤)
    let max_delta = aim_speed * time.delta_secs();
    let applied_delta = delta.clamp(-max_delta, max_delta);

    // 새로운 회전 적용 (X축에만)
    g_trans.rotation = Quat::from_euler(EulerRot::XYZ, current_x + applied_delta, 0.0, current_z);
}
