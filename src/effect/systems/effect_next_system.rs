use bevy::prelude::*;

use crate::effect::structs::{Effect, EffectMesh};

pub fn effect_next_system(
    mut commands: Commands,
    mut effects: Query<(&mut Transform, &mut Effect, Entity, &Children), With<Effect>>,
    mut effect_meshs: Query<&mut Transform, (With<EffectMesh>, Without<Effect>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    for (mut trans, mut effect, entity, children) in effects.iter_mut() {
        let color = effect.color.clone();
        let mut effect_mesh_trans = None;

        for &child in children {
            effect_mesh_trans = Some(effect_meshs.get_mut(child).unwrap());
        }

        effect.next(
            &time,
            &mut trans,
            &color,
            &mut commands,
            entity,
            &mut materials,
            &mut effect_mesh_trans.unwrap(),
        );
    }
}
