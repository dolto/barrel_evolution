use std::time::Duration;

use bevy::prelude::*;

use crate::{
    enemy::structs::Enemy,
    enemy::systems::{ENEMY_MAX_X, ENEMY_MIN_X},
    gun::{
        barrel::{Barrel, Bullet},
        systems::DESPAWN_BULLETS_Z,
    },
};

pub fn normal_enemy_spawn(
    commands: &mut Commands,
    wave: usize,
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
    bullet_mesh: Handle<Mesh>,
    bullet_color: Handle<ColorMaterial>,
) {
    let z = (DESPAWN_BULLETS_Z * 0.5 * fastrand::f32()).clamp(500., DESPAWN_BULLETS_Z * 0.5);

    let y = 300. * fastrand::f32() - 100.;
    let mut dir = [
        fastrand::f32() * 2. - 1.,
        fastrand::f32() * 2. - 1.,
        fastrand::f32() * 2. - 1.,
    ];
    dir.sort_by(|a, &b| b.partial_cmp(a).unwrap());
    let mut dir = Vec3::from_array(dir).normalize();
    dir.z /= 3.;
    dir = dir.normalize();
    let x = if dir.x < 0. { ENEMY_MAX_X } else { ENEMY_MIN_X };
    let mut trans = Transform::from_xyz(x, y, z);
    let target = trans.translation + dir;
    let dir = (target - trans.translation).normalize();
    let hp = 1000. + wave as f32;
    let speed = 30. * (wave as f32 + fastrand::f32());
    trans.rotation = Quat::from_rotation_arc(Vec3::Y, dir);
    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        trans,
        Enemy {
            hp,
            max_hp: hp,
            speed,
            max_speed: speed,
            barrel: Barrel {
                power: 5.,
                hp: 0.,
                hp_step: 0.,
                max_hp: 1.,
                reload: true,
                is_broken: false,
                bullet: Bullet {
                    up: Dir3::Z,
                    damage: 1.,
                    speed: 30. * (wave as f32 + fastrand::f32()),
                    bullet_effect: Vec::new(),
                    is_enemy: true,
                    size: 0.2,
                    mesh: bullet_mesh,
                    material: bullet_color,
                },
            },
            at_speed: Timer::new(Duration::from_millis(1000), TimerMode::Repeating),
            at_dist: 0.,
            at_dist_step: 1.,
            direction: dir,
            size_side: 5.,
            size_deep: 100.,
        },
    ));
}
