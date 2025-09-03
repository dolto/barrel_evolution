pub mod effect_check_maker;
pub mod effect_next_system;
pub mod effect_scale_system;

use bevy::prelude::*;
pub use effect_check_maker::*;
pub use effect_next_system::*;
pub use effect_scale_system::*;

use crate::effect::structs::effect_model_setup;

pub struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (effect_model_setup,)).add_systems(
            Update,
            (effect_next_system, effect_check_maker, effect_scale_system),
        );
    }
}
