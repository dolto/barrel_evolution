mod gun;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use gun::barrel::*;
use gun::gun::*;
use std::f32;

use crate::gun::systems::*;

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
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    commands
        .spawn((
            GunAimCircle,
            Transform {
                translation: Vec3::ZERO,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Sprite {
                    custom_size: Some(Vec2::new(4., 2.)),
                    color: Color::srgb(0.7, 0.1, 0.1),
                    ..Default::default()
                },
                Transform::from_xyz(-4., 6., 0.),
            ));
            parent.spawn((
                Sprite {
                    custom_size: Some(Vec2::new(2., 4.)),
                    color: Color::srgb(0.7, 0.1, 0.1),
                    ..Default::default()
                },
                Transform::from_xyz(-5., 5., 0.),
            ));

            parent.spawn((
                Sprite {
                    custom_size: Some(Vec2::new(4., 2.)),
                    color: Color::srgb(0.7, 0.1, 0.1),
                    ..Default::default()
                },
                Transform::from_xyz(4., 6., 0.),
            ));
            parent.spawn((
                Sprite {
                    custom_size: Some(Vec2::new(2., 4.)),
                    color: Color::srgb(0.7, 0.1, 0.1),
                    ..Default::default()
                },
                Transform::from_xyz(5., 5., 0.),
            ));

            parent.spawn((
                Sprite {
                    custom_size: Some(Vec2::new(4., 2.)),
                    color: Color::srgb(0.7, 0.1, 0.1),
                    ..Default::default()
                },
                Transform::from_xyz(4., -6., 0.),
            ));
            parent.spawn((
                Sprite {
                    custom_size: Some(Vec2::new(2., 4.)),
                    color: Color::srgb(0.7, 0.1, 0.1),
                    ..Default::default()
                },
                Transform::from_xyz(5., -5., 0.),
            ));

            parent.spawn((
                Sprite {
                    custom_size: Some(Vec2::new(4., 2.)),
                    color: Color::srgb(0.7, 0.1, 0.1),
                    ..Default::default()
                },
                Transform::from_xyz(-4., -6., 0.),
            ));
            parent.spawn((
                Sprite {
                    custom_size: Some(Vec2::new(2., 4.)),
                    color: Color::srgb(0.7, 0.1, 0.1),
                    ..Default::default()
                },
                Transform::from_xyz(-5., -5., 0.),
            ));
        });
    commands
        .spawn((
            Gun {
                speed: f32::to_radians(720.),
                barrels: vec![
                    Barrel {
                        power: 3.0,
                        hp: 0.,
                        max_hp: 10.,
                        damage: 0.5,
                        reload: true,
                        bullet_speed: 100.,
                    },
                    Barrel {
                        power: 3.03,
                        hp: 0.,
                        max_hp: 10.,
                        damage: 0.5,
                        reload: true,
                        bullet_speed: 100.,
                    },
                    Barrel {
                        power: 3.03,
                        hp: 0.,
                        max_hp: 10.,
                        damage: 0.5,
                        reload: true,
                        bullet_speed: 100.,
                    },
                ],
                radius: 5.,
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
