use bevy::prelude::*;

use crate::{Enemy, gun::barrel::Bullet};

pub fn bullet_move_system(mut bullet_query: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut b_trans, bullet) in bullet_query.iter_mut() {
        let up = b_trans.rotation * Vec3::Y;
        b_trans.translation += up * bullet.speed * time.delta_secs();
    }
}
