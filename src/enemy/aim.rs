use bevy::prelude::*;

use crate::util::make_rect_mesh;

#[derive(Component)]
pub struct Aim;

#[derive(Resource)]
pub struct AimSprite {
    pub mesh: Handle<Mesh>,
    pub material: Handle<ColorMaterial>,
}

impl AimSprite {
    pub fn setup(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        let parts: Vec<(f32, f32, f32, f32)> = vec![
            (-4.0, 6.0, 4.0, 2.0),
            (-5.0, 5.0, 2.0, 4.0),
            (4.0, 6.0, 4.0, 2.0),
            (5.0, 5.0, 2.0, 4.0),
            (4.0, -6.0, 4.0, 2.0),
            (5.0, -5.0, 2.0, 4.0),
            (-4.0, -6.0, 4.0, 2.0),
            (-5.0, -5.0, 2.0, 4.0),
        ];

        let base_color = Color::srgb(0.7, 0.1, 0.1);

        let mesh = make_rect_mesh(&parts, Color::WHITE);

        let mesh_handle = meshes.add(mesh);
        let material = materials.add(base_color);

        AimSprite {
            mesh: mesh_handle,
            material: material,
        }
    }
}

pub fn aim_sprite_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(AimSprite::setup(&mut meshes, &mut materials));
}
