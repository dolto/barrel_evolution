use bevy::prelude::*;

use crate::gun::{
    barrel::BarrelSprite,
    gun::{Gun, GunControlStatus},
};

pub fn fire_system(
    gun: Single<(&mut Gun, &GlobalTransform)>,
    barrels: Query<(&BarrelSprite, &GlobalTransform)>,
    mut gun_status: ResMut<GunControlStatus>,
    mut commands: Commands,
) {
    if !gun_status.aiming || !gun_status.firing {
        return;
    }
    let (mut gun, g_global_trans) = gun.into_inner();

    for (index, global_trans) in barrels {
        let local_pos =
            g_global_trans.compute_matrix().inverse() * global_trans.translation().extend(1.0);
        let local_x = local_pos.x;
        if local_x > 0. && gun.barrels[index.index].reload {
            gun.barrels[index.index].reload = false;
            let bullet = gun.barrels[index.index].fire(
                &gun,
                gun_status.aim_position.y,
                &mut gun_status,
                g_global_trans,
            );

            let barrel_offset = Vec3::new(gun.radius, 20., 0.); // 총열 위치 (총 로컬 좌표 기준)
            let spawn_pos =
                g_global_trans.rotation() * barrel_offset + g_global_trans.translation();

            commands.spawn((
                bullet,
                Sprite {
                    custom_size: Some(Vec2::new(1.1, 5.1)),
                    color: Color::srgb(1., 0.2, 0.2),
                    ..default()
                },
                Transform {
                    translation: spawn_pos,
                    rotation: g_global_trans.rotation(),
                    ..default()
                },
            ));
        } else if local_x < 0. && !gun.barrels[index.index].reload {
            gun.barrels[index.index].reload = true;
        }
    }
}
