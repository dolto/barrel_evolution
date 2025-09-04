use std::f32;

use bevy::prelude::*;

use crate::{
    effect::structs::{EffectMaker, EffectModel},
    gun::{
        barrel::{BarrelModel, BarrelSprite},
        gun::{Gun, GunSpin},
    },
};

pub fn check_gun_barrels_position(
    mut commands: Commands,
    gun_query: Single<&Gun, Or<(Changed<Gun>, Added<Gun>)>>,
    berrals_query: Query<(&BarrelSprite, Entity)>,
    gun_spin: Single<Entity, With<GunSpin>>,
    mut b_count: Local<usize>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    barrel_model: Res<BarrelModel>,
    effect_model: Res<EffectModel>,
) {
    let gun = gun_query.into_inner();
    if *b_count == gun.barrels.len() {
        return;
    }
    *b_count = gun.barrels.len();

    for (barrel_sprite, barrel) in berrals_query.iter() {
        // 여기서 barrel_sprite에 있는 Handle<ColorMaterial>을 이용해서 독립 머터리얼 제거
        materials.remove(&barrel_sprite.material);
        commands.entity(barrel).despawn();
    }

    let n = *b_count as f32;
    let mut barrel_entities = Vec::with_capacity(*b_count);
    let color = Color::srgb(0.446, 0.446, 0.446);

    for (i, _) in gun.barrels.iter().enumerate() {
        // 어떤 총열이냐에 따라서 다르게 넣어야함... 총열 구분 필요
        let fire_effect_test = EffectMaker {
            count: 3..10,
            start_dir: Vec3::new(-0.3, 80., 0.1)..Vec3::new(0.3, 100., 0.8),
            end_dir: Vec3::new(-0.3, 10., 0.1)..Vec3::new(0.3, 60., 0.8),
            start_scale: (1.)..(2.),
            end_scale: (0.)..(0.2),
            start_color: Vec4::new(0.3, 0.8, 0.1, 1.)..Vec4::new(1., 0.8, 0.8, 1.),
            end_color: Vec4::new(0.3, 0.8, 0.1, 1.)..Vec4::new(1., 0.8, 0.8, 1.),
            max_time: 0.2..0.5,
            rotate: Vec3::new(-1., 0., -0.1)..Vec3::new(1., 0., 0.1),
            meshes: vec![effect_model.dot.clone()],
            make_flag: false,
            offset_pos: Vec3::new(0., 10., 0.),
        };
        let angle = 2.0 * f32::consts::PI * (i as f32) / n;

        let x = gun.radius * angle.cos();
        let y = 20.;
        let z = gun.radius * angle.sin();

        // 총열 모델 선택 로직 필요
        let model = barrel_model.prototype.clone();
        let material = materials.add(color);

        let barrel_entity = commands
            .spawn((
                // Sprite를 Mesh로 변경
                Mesh2d(model.clone()),
                MeshMaterial2d(material.clone()),
                Transform {
                    translation: Vec3::new(x, y, z),
                    ..default()
                },
                fire_effect_test,
                BarrelSprite {
                    index: i,
                    material: material.clone(),
                },
            ))
            .id();

        barrel_entities.push(barrel_entity);
    }

    commands
        .entity(gun_spin.into_inner())
        .add_children(&barrel_entities);
}
