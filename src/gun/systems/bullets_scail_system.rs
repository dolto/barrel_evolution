use bevy::prelude::*;

use crate::gun::{barrel::Bullet, systems::DESPAWN_BULLETS_Z};

pub fn bullets_scail_system(
    mut bullets_query: Query<(&GlobalTransform, &mut Transform), With<Bullet>>,
) {
    for (bullet_global, mut bullet_trans) in bullets_query.iter_mut() {
        let scale = (DESPAWN_BULLETS_Z - bullet_global.translation().z) / DESPAWN_BULLETS_Z;
        bullet_trans.scale = Vec3::new(scale, scale, scale);
    }
}
