use bevy::prelude::*;

use crate::gun::{barrel::Bullet, systems::DESPAWN_BULLETS_Z};

pub fn despawn_bullets_system(
    mut commands: Commands,
    bullets: Query<(Entity, &Transform), With<Bullet>>,
    window: Single<&Window>,
) {
    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;

    for (entity, trans) in bullets.iter() {
        let pos = trans.translation;

        // 화면 밖 혹은 Z축 범위를 넘어갔는지 체크
        if pos.x < -half_width
            || pos.x > half_width
            || pos.y < -half_height
            || pos.y > half_height
            || pos.z < -DESPAWN_BULLETS_Z
            || pos.z > DESPAWN_BULLETS_Z
        {
            commands.entity(entity).despawn();
        }
    }
}
