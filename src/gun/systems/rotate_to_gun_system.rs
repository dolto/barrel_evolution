use bevy::prelude::*;

use crate::gun::gun::{Gun, GunControlStatus};

pub fn rotate_to_gun_system(gun: Single<(&Gun, &mut Transform)>, aim: Res<GunControlStatus>) {
    let (_, mut g_trans) = gun.into_inner();
    let gun_pos = g_trans.translation;
    let aim_pos = aim.aim_position.extend(gun_pos.z); // z는 동일 평면 가정

    // --- 목표 방향 ---
    let dir = (aim_pos - gun_pos).normalize();

    // -200 ~ 250 이 0 ~ 40으로 변경
    let pitch_deg = (1.0 - ((aim_pos.y + 200.0) / 450.0)).clamp(0.0, 1.0);
    let pitch = (60.0 * pitch_deg).to_radians();

    // yaw(좌우, Z축 회전)
    let yaw = dir.y.atan2(dir.x);
    // 여기에 pitch가 60도에 가까워 질수록 중앙으로 가는 보정
    let zrol = (yaw - std::f32::consts::FRAC_PI_2) * (0.6 + 0.4 * (1. - pitch_deg.powf(1.52)));

    // roll (총 자체 X축 회전)
    let roll = 0.0;

    // 회전 적용
    let target_rot =
        Quat::from_euler(EulerRot::XYZ, pitch, 0.0, zrol) * Quat::from_rotation_x(roll);

    g_trans.rotation = target_rot;
}
