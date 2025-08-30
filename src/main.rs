mod gun;
mod util;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use gun::barrel::*;
use gun::gun::*;
use std::f32;
use std::time::Duration;

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
            GunPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                enemy_move_system,
                enemy_scale_system,
                aim_circle_detecting_system,
            ),
        )
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
) {
    commands.spawn(Camera2d::default());

    spawn_aim(&mut commands, &mut meshes, &mut materials, &aim_sprite);

    enemy_spawn(&mut commands, 1);

    let bullet_test = Bullet {
        up: Dir3::Y,
        damage: 0.5,
        speed: 400.,
        bullet_effect: Vec::new(),
        is_enemy: false,
        size: 0.2,
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
const ENEMY_MAX_X: f32 = 200.5;
const ENEMY_MIN_X: f32 = -200.5;

const ENEMY_MAX_Y: f32 = 250.;
const ENEMY_MIN_Y: f32 = -100.;

const ENEMY_MAX_Z: f32 = DESPAWN_BULLETS_Z * 0.5;
const ENEMY_MIN_Z: f32 = 0.;

#[derive(Component)]
pub struct Enemy {
    pub hp: f32,         // 체력
    pub speed: f32,      // 이동속도
    pub barrel: Barrel,  // 공격 수단
    pub at_speed: Timer, // 공격 빈도
    pub direction: Vec3, // 이동 방향
    pub at_dist: f32,    // 공격 사거리(시간이 지날수록 커져서, 이내 공격하게됨)
    pub size_side: f32,  // 피격 범위(2d)
    pub size_deep: f32,  // 피격 범위(z축)
}

impl Enemy {
    pub fn dead(&self, commands: &mut Commands, entity: Entity) {
        // 여기에 죽음 이펙트 추가
        commands.entity(entity).despawn();
    }
}

#[derive(Resource)]
pub struct EnemyMeshse {
    meshes: Vec<Handle<Mesh>>,
}

#[derive(Resource)]
pub struct EnemyWave {
    level: usize,
}

pub fn enemy_spawn(commands: &mut Commands, wave: usize) {
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
    trans.rotation = Quat::from_rotation_arc(Vec3::Y, dir);
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::splat(5.)),
            color: Color::WHITE,
            ..default()
        },
        trans,
        Enemy {
            hp: 1000. + wave as f32,
            speed: 30. * (wave as f32 + fastrand::f32()),
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
                },
            },
            at_speed: Timer::new(Duration::from_millis(1000), TimerMode::Repeating),
            at_dist: 0.,
            direction: dir,
            size_side: 10.,
            size_deep: 100.,
        },
    ));
}

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
        enemy.direction = dir.normalize_or_zero();

        // 바라보는 방향 맞춤
        if enemy.direction.length_squared() > 0.0 {
            enemy_trans.rotation = Quat::from_rotation_arc(Vec3::Y, enemy.direction);
        }

        // 이동
        enemy_trans.translation += enemy.direction * enemy.speed * dt;
    }
}
pub fn enemy_scale_system(mut enemys: Query<&mut Transform, With<Enemy>>) {
    for mut enemy_trans in enemys.iter_mut() {
        let enemy_pos = enemy_trans.translation;
        let scale = ((enemy_pos.y + 200.) / 450.) * 2.5 + 0.5;
        enemy_trans.scale = Vec3::splat(scale);
    }
}

pub fn aim_circle_detecting_system(
    mut gun_status: ResMut<GunControlStatus>,
    gun_trans: Single<&GlobalTransform, With<Gun>>,
    enemys: Query<(&Transform, &Enemy), With<Enemy>>,
) {
    let mut temp = false;
    for (enemy_trans, enemy) in enemys {
        let dist = (enemy_trans.translation - gun_trans.translation())
            .dot(gun_trans.rotation() * Vec3::NEG_Z)
            .abs();

        if dist <= enemy.size_deep {
            temp = true;
        }
    }

    gun_status.is_enemy_z = temp;
}
