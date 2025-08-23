use bevy::input::touch::Touch;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use std::f32;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Fixed Mobile Window".into(),
                resolution: WindowResolution::new(405.0, 720.0),
                resizable: false, // 크기 변경 금지
                ..default()
            }),
            ..default()
        }),))
        .insert_resource(GunControlStatus {
            aiming: false,
            firing: false,
            aim_position: Vec2::ZERO,
        })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                check_gun_barrels_position,
                rotate_gun_system,
                fix_barrel_rotation_system,
                fire_system,
                update_aim_position,
                rotate_to_gun_system,
                update_gun_control_status,
                bullet_move_system,
                despawn_bullets_system,
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
    bullet_speed: f32,
}

impl Barrel {
    pub fn fire(
        &self,
        gun: &Gun,
        y: f32,
        g_trans: &mut Transform,
        g_global: &GlobalTransform,
    ) -> Bullet {
        let up = g_global.up();
        let recoil = (Vec3::new(fastrand::f32() * 2. - 1., 0., fastrand::f32() * 2. - 1.)
            .normalize()
            * self.power)
            * (1. - gun.recoil_control);
        g_trans.rotation *= Quat::from_euler(EulerRot::XYZ, recoil.x, recoil.y, recoil.z);
        Bullet {
            y,
            up,
            damage: self.damage,
            speed: self.bullet_speed,
        }
    }
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
    recoil_control: f32, // 반동을 제어하는 정도
}

#[derive(Resource)]
struct GunControlStatus {
    aiming: bool, // 마우스를 누르고 있는 동안 true
    firing: bool, // 클릭 토글
    aim_position: Vec2,
}

#[derive(Component)]
struct Bullet {
    y: f32,
    up: Dir3,
    damage: f32,
    speed: f32,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    // 중심
    commands
        .spawn((
            Gun {
                speed: 100.2,
                barrels: vec![
                    Barrel {
                        power: 0.03,
                        hp: 0.,
                        max_hp: 10.,
                        damage: 0.5,
                        reload: true,
                        bullet_speed: 100.,
                    },
                    Barrel {
                        power: 0.03,
                        hp: 0.,
                        max_hp: 10.,
                        damage: 0.5,
                        reload: true,
                        bullet_speed: 100.,
                    },
                    Barrel {
                        power: 0.03,
                        hp: 0.,
                        max_hp: 10.,
                        damage: 0.5,
                        reload: true,
                        bullet_speed: 100.,
                    },
                ],
                radius: 5.,
                aim_speed: f32::to_radians(35.),
                recoil_control: 0.,
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

fn fire_system(
    gun: Single<(&mut Gun, &GlobalTransform, &mut Transform)>,
    barrels: Query<(&BarrelSprite, &GlobalTransform)>,
    gun_status: Res<GunControlStatus>,
    mut commands: Commands,
) {
    if !gun_status.aiming || !gun_status.firing {
        return;
    }
    let (mut gun, g_global_trans, mut g_trans) = gun.into_inner();

    for (index, global_trans) in barrels {
        let local_pos =
            g_global_trans.compute_matrix().inverse() * global_trans.translation().extend(1.0);
        let local_x = local_pos.x;
        if local_x > 0. && gun.barrels[index.index].reload {
            gun.barrels[index.index].reload = false;
            let bullet = gun.barrels[index.index].fire(
                &gun,
                gun_status.aim_position.y,
                &mut g_trans,
                g_global_trans,
            );

            let barrel_offset = Vec3::new(gun.radius, 20., 0.); // 총열 위치 (총 로컬 좌표 기준)
            let spawn_pos =
                g_global_trans.rotation() * barrel_offset + g_global_trans.translation();

            commands.spawn((
                bullet,
                Sprite {
                    custom_size: Some(Vec2::new(1., 5.)),
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

fn update_aim_position(
    window: Single<&Window>,
    touches: Res<Touches>,
    camera: Single<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut gun_control_status: ResMut<GunControlStatus>,
) {
    if !gun_control_status.aiming {
        return;
    }
    let (camera, camera_trans) = camera.into_inner();
    let mut aim_positon = Vec2::ZERO;
    let mut is_pos = false;
    if let Some(pos) = window.cursor_position() {
        aim_positon = pos;
        is_pos = true;
    }
    for touch in touches.iter() {
        aim_positon = touch.position();
        is_pos = true;
    }

    let aim = camera.viewport_to_world_2d(camera_trans, aim_positon);

    if let Ok(pos) = aim {
        if is_pos {
            gun_control_status.aim_position = pos;
        }
    }
}

fn rotate_to_gun_system(
    time: Res<Time>,
    window: Single<&Window>,
    gun: Single<(&Gun, &mut Transform)>,
    aim: Res<GunControlStatus>,
) {
    let (gun, mut g_trans) = gun.into_inner();
    let g_tal = g_trans.translation;
    let aim_pos = aim.aim_position;
    let aim_speed = gun.aim_speed;

    // --- Z축 회전 (기존) ---
    let gun_forward = (g_trans.rotation * Vec3::Y).xy();
    let to_aim = (aim_pos - g_tal.xy()).normalize();
    let forward_dot_aim = gun_forward.dot(to_aim);
    let gun_right = (g_trans.rotation * Vec3::X).xy();
    let right_dot_aim = gun_right.dot(to_aim);
    let rotation_sign = -f32::copysign(1.0, right_dot_aim);
    let max_angle = f32::acos(forward_dot_aim.clamp(-1.0, 1.0));
    let rotation_angle = rotation_sign * (aim_speed * time.delta_secs()).min(max_angle);
    g_trans.rotate_z(rotation_angle);

    // --- X축 회전 (속도 제한 적용) ---
    let window = window.into_inner();
    let win_height = window.height() / 2.;
    let screen_y = win_height - aim_pos.y.clamp(-win_height, win_height);
    let target_x_deg = (screen_y / window.height()) * 65.0;
    let target_x_rad = target_x_deg.to_radians();

    // 현재 X축 회전
    let (current_x, _, current_z) = g_trans.rotation.to_euler(EulerRot::XYZ);

    // 목표 각도와 현재 각도 차이
    let delta = target_x_rad - current_x;

    // 제한된 회전량 (aim_speed에 맞춤)
    let max_delta = aim_speed * time.delta_secs();
    let applied_delta = delta.clamp(-max_delta, max_delta);

    // 새로운 회전 적용 (X축에만)
    g_trans.rotation = Quat::from_euler(EulerRot::XYZ, current_x + applied_delta, 0.0, current_z);
}

fn update_gun_control_status(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut status: ResMut<GunControlStatus>,
    mut last_touch: Local<Option<Touch>>, // 마지막 활성 터치 저장
) {
    // firing 토글 (마우스/터치 둘 다에서 발생)
    if mouse_buttons.just_pressed(MouseButton::Left) || touches.any_just_pressed() {
        status.firing = !status.firing;
    }

    // aiming: 마우스 누르거나 터치가 있으면 true
    let mut aiming = mouse_buttons.pressed(MouseButton::Left);

    if touches.iter().next().is_some() {
        aiming = true;

        // 마지막 눌린 터치를 저장
        if let Some(finger) = touches.iter().max_by_key(|f| f.id()) {
            *last_touch = Some(finger.clone());
        }
    } else {
        *last_touch = None;
    }

    status.aiming = aiming;
}

fn bullet_move_system(mut bullet_query: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut b_trans, bullet) in bullet_query.iter_mut() {
        let up = b_trans.rotation * Vec3::Y;
        b_trans.translation += up * bullet.speed * time.delta_secs();
    }
}

fn despawn_bullets_system(
    mut commands: Commands,
    bullets: Query<(Entity, &Transform), With<Bullet>>,
    window: Single<&Window>,
) {
    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;

    for (entity, trans) in bullets.iter() {
        let pos = trans.translation;

        // 화면 밖 혹은 Z축 범위를 넘어갔는지 체크
        if pos.x < -half_width
            || pos.x > half_width
            || pos.y < -half_height
            || pos.y > half_height
            || pos.z < -10000.0
            || pos.z > 10000.0
        {
            commands.entity(entity).despawn();
        }
    }
}
