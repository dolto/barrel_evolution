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
