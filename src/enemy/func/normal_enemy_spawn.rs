use std::time::Duration;

use bevy::{prelude::*, sprite::Material2d};

use crate::{
    effect::structs::EffectMaker,
    enemy::{
        aim::{Aim, AimSprite},
        structs::Enemy,
        systems::{ENEMY_MAX_X, ENEMY_MIN_X},
    },
    gun::{
        barrel::{Barrel, Bullet},
        systems::DESPAWN_BULLETS_Z,
    },
    util::DeadFlag,
};

pub fn normal_enemy_spawn(
    commands: &mut Commands,
    wave: usize,
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
    bullet_mesh: Handle<Mesh>,
    bullet_color: Handle<ColorMaterial>,
    aim_sprite: &AimSprite,
    effect_mesh: Vec<Handle<Mesh>>,
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
    commands
        .spawn((
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
            DeadFlag(false),
            EffectMaker {
                count: 1..5,
                start_dir: Vec3::splat(-100.)..Vec3::splat(100.),
                end_dir: Vec3::splat(-1000.)..Vec3::splat(1000.),
                // 밝은 노란색 계열 (RGB 대략 0.8~1.0, G는 살짝 낮춰도 됨)
                start_color: Vec4::new(0.9, 0.9, 0.2, 1.0)..Vec4::new(1.0, 1.0, 0.4, 1.0),
                // 주황색 계열 (R 높고, G 중간, B 낮음)
                end_color: Vec4::new(1.0, 0.6, 0.0, 1.0)..Vec4::new(1.0, 0.4, 0.0, 1.0),
                start_scale: (3.0)..6.,
                end_scale: (0.)..0.1,
                max_time: (0.3)..(1.2),
                rotate: Vec3::splat(-10.)..Vec3::splat(10.),
                meshes: effect_mesh,
                make_flag: false,
            },
        ))
        .with_child((
            Transform::default(),
            Mesh2d(aim_sprite.mesh.clone()),
            MeshMaterial2d(aim_sprite.material.clone()),
            Visibility::Hidden,
            Aim,
        ));
}
