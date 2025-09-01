mod aim_circle_color_system;
mod aim_circle_detecting_system;
mod bullet_hit_system;
mod bullet_move_system;
mod bullets_scail_system;
mod check_gun_barrels_position;
mod despawn_bullets_system;
mod fire_system;
mod fix_barrel_rotation_system;
mod gun_cooling_color_system;
mod gun_cooling_system;
mod rotate_gun_system;
mod rotate_to_gun_system;
mod update_aim_circle_position;
mod update_aim_position;
mod update_gun_control_status;

use aim_circle_color_system::*;
use aim_circle_detecting_system::*;
use bevy::prelude::*;
use bullet_hit_system::*;
use bullet_move_system::*;
use bullets_scail_system::*;
use check_gun_barrels_position::*;
use despawn_bullets_system::*;
use fire_system::*;
use fix_barrel_rotation_system::*;
use gun_cooling_color_system::*;
use gun_cooling_system::*;
use rotate_gun_system::*;
use rotate_to_gun_system::*;
use update_aim_circle_position::*;
use update_aim_position::*;
use update_gun_control_status::*;

use crate::gun::{
    barrel::{barrel_model_setup, bullet_model_setup},
    gun::GunControlStatus,
};

pub const DESPAWN_BULLETS_Z: f32 = 1500.;

pub struct GunPlugin;

impl Plugin for GunPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(GunControlStatus {
            aiming: false,
            firing: false,
            aim_position: Vec2::ZERO,
        })
        .add_systems(Startup, (barrel_model_setup, bullet_model_setup))
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
                bullets_scail_system,
                // update_aim_circle_position,
                // aim_circle_color_system,
                gun_cooling_system,
                bullet_hit_system,
                gun_cooling_color_system,
                aim_circle_detecting_system,
            ),
        );
    }
}
