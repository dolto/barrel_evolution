use std::f32;

use bevy::prelude::*;

use crate::gun::{
    barrel::BarrelSprite,
    gun::{Gun, GunSpin},
};

pub fn check_gun_barrels_position(
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
