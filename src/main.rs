use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};
use std::f32;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Fixed Mobile Window".into(),
                resolution: WindowResolution::new(405.0, 720.0),
                resizable: false, // 크기 변경 금지
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                check_gun_barrels_position,
                rotate_gun_system,
                cursor_to_world_2d,
                fix_barrel_rotation_system,
                fire_system,
            ),
        )
        .run();
}

struct Barrel {
    power: f32,   // 반동
    hp: f32,      // 내구도 높을수록 안좋음
    max_hp: f32,  // 최대 내구도
    damage: f32,  // 공격력
    reload: bool, // 발사가능
}
#[derive(Component)]
struct BarrelSprite {
    index: usize,
}

#[derive(Component)]
struct GunSpin;

#[derive(Component)]
struct Gun {
    speed: f32,           // 회전속도
    barrels: Vec<Barrel>, // 총열 entity모음
    radius: f32,          // 반지름
    aim_speed: f32,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    // 중심
    commands
        .spawn((
            Gun {
                speed: 1.2,
                barrels: vec![
                    Barrel {
                        power: 0.3,
                        hp: 0.,
                        max_hp: 10.,
                        damage: 0.5,
                        reload: true,
                    },
                    Barrel {
                        power: 0.3,
                        hp: 0.,
                        max_hp: 10.,
                        damage: 0.5,
                        reload: true,
                    },
                    Barrel {
                        power: 0.3,
                        hp: 0.,
                        max_hp: 10.,
                        damage: 0.5,
                        reload: true,
                    },
                ],
                radius: 5.,
                aim_speed: f32::to_radians(35.),
            },
            Transform {
                translation: Vec3::new(0., -320., 0.),
                ..default()
            },
        ))
        .with_child((
            Transform {
                translation: Vec3::ZERO,
                ..default()
            },
            GunSpin,
        ));
}

fn check_gun_barrels_position(
    mut commands: Commands,
    gun_query: Single<&Gun, Or<(Changed<Gun>, Added<Gun>)>>,
    berrals_query: Query<Entity, With<BarrelSprite>>,
    gun_spin: Single<Entity, With<GunSpin>>,
    mut b_count: Local<usize>,
) {
    let gun = gun_query.into_inner();
    if *b_count == gun.barrels.len() {
        return;
    }
    *b_count = gun.barrels.len();

    for child in berrals_query.iter() {
        commands.entity(child).despawn();
    }

    let n = *b_count as f32;
    let mut barrel_entities = Vec::with_capacity(*b_count);

    for (i, _) in gun.barrels.iter().enumerate() {
        let angle = 2.0 * f32::consts::PI * (i as f32) / n;

        let x = gun.radius * angle.cos();
        let y = 20.;
        let z = gun.radius * angle.sin();

        let barrel_entity = commands
            .spawn((
                Sprite {
                    color: Color::srgb_u8(119, 119, 119),
                    custom_size: Some(Vec2::new(2., 20.)),
                    ..default()
                },
                Transform {
                    translation: Vec3::new(x, y, z),
                    ..default()
                },
                BarrelSprite { index: i },
            ))
            .id();

        barrel_entities.push(barrel_entity);
    }

    commands
        .entity(gun_spin.into_inner())
        .add_children(&barrel_entities);
}

fn fix_barrel_rotation_system(
    time: Res<Time>,
    gun: Single<&Gun>,
    mut barrel_query: Query<&mut Transform, With<BarrelSprite>>,
) {
    for mut b_trans in barrel_query.iter_mut() {
        b_trans.rotate_y(-gun.speed * time.delta_secs());
    }
}

fn rotate_gun_system(
    gun: Single<&Gun>,
    time: Res<Time>,
    gun_spin: Single<&mut Transform, With<GunSpin>>,
) {
    gun_spin
        .into_inner()
        .rotate_y(gun.speed * time.delta_secs());
}

fn cursor_to_world_2d(
    primary_window: Single<&Window, With<PrimaryWindow>>,
    camera_query: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    gun: Single<(&Gun, &mut Transform), With<Gun>>,
    time: Res<Time>,
) {
    let (camera, camera_transform) = camera_query.into_inner();
    let (gun, mut g_trans) = gun.into_inner();

    if let Some(cursor_position) = primary_window.cursor_position() {
        // -------------------------------
        // 1️⃣ Z 회전: 총이 마우스를 바라보게
        // -------------------------------
        let world_position = camera
            .viewport_to_world_2d(camera_transform, cursor_position)
            .unwrap();

        let gun_forward = (g_trans.rotation * Vec3::Y).xy();
        let to_cursor = (world_position - g_trans.translation.xy()).normalize();
        let forward_dot_cursor = gun_forward.dot(to_cursor);

        if (forward_dot_cursor - 1.0).abs() < f32::EPSILON {
            return;
        }

        let gun_right = (g_trans.rotation * Vec3::X).xy();
        let right_dot_cursor = gun_right.dot(to_cursor);

        let rotation_sign = -f32::copysign(1.0, right_dot_cursor);
        let max_angle = f32::acos(forward_dot_cursor.clamp(-1.0, 1.0));
        let rotation_angle = rotation_sign * (gun.aim_speed * time.delta_secs()).min(max_angle);

        g_trans.rotate_z(rotation_angle);

        // -------------------------------
        // 2️⃣ X 회전: 화면 위아래에 따라 0~45도
        // -------------------------------
        let window_size = Vec2::new(
            primary_window.width() as f32,
            primary_window.height() as f32,
        );
        let normalized_y = (cursor_position.y / window_size.y).clamp(0.0, 1.0);
        let target_x = normalized_y * 60.0_f32.to_radians();

        // 현재 X 회전 가져오기
        let (current_x, _, current_z) = g_trans.rotation.to_euler(EulerRot::XYZ);
        let diff_x = target_x - current_x;

        // aim_speed 기반으로 부드럽게 적용
        let rotation_step_x = (gun.aim_speed * time.delta_secs()).min(diff_x.abs());
        let new_x = current_x + diff_x.signum() * rotation_step_x;

        // 최종 회전 적용 (X 회전 업데이트, Z는 그대로)
        g_trans.rotation = Quat::from_euler(EulerRot::XYZ, new_x, 0.0, current_z);
    }
}

fn fire_system(
    gun: Single<(&mut Gun, &GlobalTransform)>,
    barrels: Query<(&BarrelSprite, &GlobalTransform)>,
) {
    let (mut gun, g_trans) = gun.into_inner();
    let fire_point = g_trans.rotation() * Vec3::new(gun.radius, 20., 0.) + g_trans.translation();
    let reload_point = g_trans.rotation() * Vec3::new(-gun.radius, 20., 0.) + g_trans.translation();
    for (index, trans) in barrels {
        if trans.translation().distance(fire_point) < 1.0 && gun.barrels[index.index].reload {
            println!("{}fire!", index.index);
            gun.barrels[index.index].reload = false;
        } else if trans.translation().distance(reload_point) < 1.0
            && !gun.barrels[index.index].reload
        {
            println!("{}reload", index.index);
            gun.barrels[index.index].reload = true;
        }
    }
}
