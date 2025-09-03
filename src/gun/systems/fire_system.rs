use bevy::prelude::*;

use crate::{
    gun::{
        barrel::BarrelSprite,
        gun::{Gun, GunControlStatus},
    },
    util::DeadFlag,
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
        if local_x > 0. && !gun.barrels[index.index].is_broken && gun.barrels[index.index].reload {
            gun.barrels[index.index].reload = false;
            gun.barrels[index.index].hp = (gun.barrels[index.index].hp
                + gun.barrels[index.index].hp_step)
                .min(gun.barrels[index.index].max_hp);

            if gun.barrels[index.index].hp == gun.barrels[index.index].max_hp {
                gun.barrels[index.index].is_broken = true;
            }
            let bullet = gun.barrels[index.index].fire(&gun, &mut gun_status, g_global_trans);
            let b_mesh = bullet.mesh.clone();
            let b_material = bullet.material.clone();

            let barrel_offset = Vec3::new(gun.radius, 20., 0.); // 총열 위치 (총 로컬 좌표 기준)
            let spawn_pos =
                g_global_trans.rotation() * barrel_offset + g_global_trans.translation();

            commands.spawn((
                bullet,
                DeadFlag(false),
                Mesh2d(b_mesh.clone()),
                MeshMaterial2d(b_material.clone()),
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
