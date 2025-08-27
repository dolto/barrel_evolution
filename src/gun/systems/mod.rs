mod bullet_move_system;
mod bullets_scail_system;
mod check_gun_barrels_position;
mod despawn_bullets_system;
mod fire_system;
mod fix_barrel_rotation_system;
mod rotate_gun_system;
mod rotate_to_gun_system;
mod update_aim_circle_position;
mod update_aim_position;
mod update_gun_control_status;

use bevy::prelude::*;
use bullet_move_system::*;
use bullets_scail_system::*;
use check_gun_barrels_position::*;
use despawn_bullets_system::*;
use fire_system::*;
use fix_barrel_rotation_system::*;
use rotate_gun_system::*;
use rotate_to_gun_system::*;
use update_aim_circle_position::*;
use update_aim_position::*;
use update_gun_control_status::*;

use crate::gun::gun::GunControlStatus;

pub const DESPAWN_BULLETS_Z: f32 = 1500.;

pub struct GunPlugin;

impl Plugin for GunPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(GunControlStatus {
            aiming: false,
            firing: false,
            aim_position: Vec2::ZERO,
            aim_speed: 60.,
        })
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
                update_aim_circle_position,
            ),
        );
    }
}
