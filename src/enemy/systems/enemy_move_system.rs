use bevy::prelude::*;

use crate::enemy::{
    structs::Enemy,
    systems::{ENEMY_MAX_X, ENEMY_MAX_Y, ENEMY_MAX_Z, ENEMY_MIN_X, ENEMY_MIN_Y, ENEMY_MIN_Z},
};

fn boundary_force(pos: f32, min: f32, max: f32) -> f32 {
    let margin = 50.0; // 경계 근처에서 힘이 작동하는 거리
    if pos > max - margin {
        -(pos - (max - margin)) / margin
    } else if pos < min + margin {
        (min + margin - pos) / margin
    } else {
        0.0
    }
}

pub fn enemy_move_system(mut enemys: Query<(&mut Transform, &mut Enemy)>, time: Res<Time>) {
    let dt = time.delta_secs();

    for (mut enemy_trans, mut enemy) in enemys.iter_mut() {
        let mut dir = enemy.direction;

        // 복원력 추가
        dir.x += boundary_force(enemy_trans.translation.x, ENEMY_MIN_X, ENEMY_MAX_X) * dt * 2.0;
        dir.y += boundary_force(enemy_trans.translation.y, ENEMY_MIN_Y, ENEMY_MAX_Y) * dt * 2.0;
        dir.z += boundary_force(enemy_trans.translation.z, ENEMY_MIN_Z, ENEMY_MAX_Z) * dt * 2.0;

        // 약한 댐핑 적용 → 출렁이며 감쇠
        dir *= 0.995; // 값이 1.0에 가까울수록 더 오래 출렁거림

        // 방향 갱신
        enemy.direction = dir.normalize();

        // 바라보는 방향 맞춤 (방향 벡터가 0이라면 회전하면 적용하면 안됨)
        if enemy.direction.length_squared() > 0.0 {
            enemy_trans.rotation = Quat::from_rotation_arc(Vec3::Y, enemy.direction);
        }

        // 이동
        enemy_trans.translation += enemy.direction * enemy.speed * dt;
    }
}
