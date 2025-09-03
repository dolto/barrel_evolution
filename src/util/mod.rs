use bevy::prelude::*;
pub mod dead_flag;
pub mod make_rect_mesh;

pub use dead_flag::*;
pub use make_rect_mesh::*;

pub struct UtilPlugin;

impl Plugin for UtilPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (check_dead_flag_system,));
    }
}
