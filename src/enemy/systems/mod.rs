pub mod enemy_move_system;
pub mod enemy_scale_system;

pub use enemy_move_system::*;
pub use enemy_scale_system::*;

use bevy::prelude::*;

use crate::{enemy::structs::enemy_mesh_setup_system, gun::systems::DESPAWN_BULLETS_Z};

pub const ENEMY_MAX_X: f32 = 200.5;
pub const ENEMY_MIN_X: f32 = -200.5;

pub const ENEMY_MAX_Y: f32 = 250.;
pub const ENEMY_MIN_Y: f32 = -100.;

pub const ENEMY_MAX_Z: f32 = DESPAWN_BULLETS_Z * 0.5;
pub const ENEMY_MIN_Z: f32 = 0.;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (enemy_mesh_setup_system,))
            .add_systems(Update, (enemy_move_system, enemy_scale_system));
    }
}
