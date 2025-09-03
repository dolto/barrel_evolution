mod effect;
mod enemy;
mod gun;
mod util;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use gun::barrel::*;
use gun::gun::*;
use std::f32;

use crate::effect::structs::EffectModel;
use crate::effect::systems::EffectPlugin;
use crate::enemy::aim::AimSprite;
use crate::enemy::func::normal_enemy_spawn;
use crate::enemy::structs::EnemyMeshes;
use crate::enemy::systems::EnemyPlugin;
use crate::gun::systems::*;
use crate::util::UtilPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Fixed Mobile Window".into(),
                    resolution: WindowResolution::new(405.0, 720.0),
                    resizable: false, // 크기 변경 금지
                    ..default()
                }),
                ..default()
            }),
            UtilPlugin,
            EffectPlugin,
            EnemyPlugin,
            GunPlugin,
        ))
        .add_systems(PostStartup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    aim_sprite: Res<AimSprite>,
    enemy_sprite: Res<EnemyMeshes>,
    bullet_meshes: Res<BulletModel>,
    effect_model: Res<EffectModel>,
) {
    commands.spawn(Camera2d::default());

    let len = enemy_sprite.normal.len();
    let eb_mesh = bullet_meshes.boom.clone();
    let eb_material = bullet_meshes.enemy_color.clone();
    normal_enemy_spawn(
        &mut commands,
        1,
        enemy_sprite.normal[fastrand::usize(0..len)].clone(),
        enemy_sprite.materials[0].clone(),
        eb_mesh,
        eb_material,
        &aim_sprite,
        vec![effect_model.dot.clone()],
    );

    let pb_mesh = bullet_meshes.base.clone();
    let pb_material = bullet_meshes.player_color_red.clone();
    let bullet_test = Bullet {
        up: Dir3::Y,
        damage: 0.5,
        speed: 400.,
        bullet_effect: Vec::new(),
        is_enemy: false,
        size: 0.2,
        mesh: pb_mesh,
        material: pb_material,
    };
    commands
        .spawn((
            Gun {
                speed: f32::to_radians(3720.),
                aim_speed: 360.,
                barrels: vec![
                    Barrel {
                        power: 3.0,
                        hp: 0.,
                        hp_step: 0.1,
                        max_hp: 10.,
                        reload: true,
                        bullet: bullet_test.clone(),
                        is_broken: false,
                    },
                    Barrel {
                        power: 3.03,
                        hp: 0.,
                        max_hp: 10.,
                        hp_step: 0.1,
                        reload: true,
                        bullet: bullet_test.clone(),
                        is_broken: false,
                    },
                    Barrel {
                        power: 3.03,
                        hp: 0.,
                        max_hp: 10.,
                        hp_step: 0.1,
                        reload: true,
                        bullet: bullet_test.clone(),
                        is_broken: false,
                    },
                ],
                radius: 5.,
                recoil_control: 0.,
                heal: 1.,
            },
            Transform {
                translation: Vec3::new(0., -320., 0.),
                ..default()
            },
            Visibility::default(),
        ))
        .with_child((
            Transform {
                translation: Vec3::ZERO,
                ..default()
            },
            GunSpin,
            Visibility::default(),
        ));
}

// 이펙트 {
//   메쉬, 컬러, 방향, 시간, 스케일, 지난 시간
//   색상 시작, 색상 끝,
//   방향 시작, 방향 끝
// }
