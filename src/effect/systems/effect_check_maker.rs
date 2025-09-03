use bevy::prelude::*;

use crate::effect::structs::EffectMaker;

pub fn effect_check_maker(
    mut commands: Commands,
    mut effect_makers: Query<(&GlobalTransform, &mut EffectMaker)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (trans, mut effect_maker) in effect_makers.iter_mut() {
        if effect_maker.make_flag {
            effect_maker.make_flag = false;
            effect_maker.spawn_effect(&mut commands, &mut materials, &trans.compute_transform());
        }
    }
}
