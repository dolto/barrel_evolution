mod enemy;
mod gun;
mod util;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use gun::barrel::*;
use gun::gun::*;
use std::f32;

use crate::enemy::func::normal_enemy_spawn;
use crate::enemy::structs::EnemyMeshes;
use crate::enemy::systems::EnemyPlugin;
use crate::gun::systems::*;
use util::*;

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
            EnemyPlugin,
            GunPlugin,
        ))
        .add_systems(PostStartup, setup)
        .run();
}

#[derive(Resource)]
struct AimSprite {
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
}

fn spawn_aim(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    aim_sprite: &Option<Res<AimSprite>>,
) {
    let mesh_handle;
    let material;

    if let Some(aim_sprite) = aim_sprite {
        mesh_handle = aim_sprite.mesh.clone();
        material = aim_sprite.material.clone();
    } else {
        // 조준원을 구성하는 파츠들(네가 준 값들)
        let parts: Vec<(f32, f32, f32, f32)> = vec![
            (-4.0, 6.0, 4.0, 2.0),
            (-5.0, 5.0, 2.0, 4.0),
            (4.0, 6.0, 4.0, 2.0),
            (5.0, 5.0, 2.0, 4.0),
            (4.0, -6.0, 4.0, 2.0),
            (5.0, -5.0, 2.0, 4.0),
            (-4.0, -6.0, 4.0, 2.0),
            (-5.0, -5.0, 2.0, 4.0),
        ];

        let base_color = Color::srgb(0.7, 0.1, 0.1);

        let mesh = make_rect_mesh(&parts, Color::WHITE);

        mesh_handle = meshes.add(mesh);
        material = materials.add(base_color);

        commands.insert_resource(AimSprite {
            mesh: mesh_handle.clone(),
            material: material.clone(),
        });
    }
    commands.spawn((
        Mesh2d(mesh_handle.clone()),
        MeshMaterial2d(material.clone()),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)).with_scale(Vec3::splat(1.0)),
        GunAimCircle,
    ));
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    aim_sprite: Option<Res<AimSprite>>,
    enemy_sprite: Res<EnemyMeshes>,
    bullet_meshes: Res<BulletModel>,
) {
    commands.spawn(Camera2d::default());

    spawn_aim(&mut commands, &mut meshes, &mut materials, &aim_sprite);

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
