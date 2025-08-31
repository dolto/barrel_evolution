use bevy::prelude::*;

use crate::{gun::barrel::Barrel, util::make_rect_mesh};

#[derive(Component)]
pub struct Enemy {
    pub hp: f32,         // 체력
    pub speed: f32,      // 이동속도
    pub barrel: Barrel,  // 공격 수단
    pub at_speed: Timer, // 공격 빈도
    pub direction: Vec3, // 이동 방향
    pub at_dist: f32,    // 공격 사거리(시간이 지날수록 커져서, 이내 공격하게됨)
    pub size_side: f32,  // 피격 범위(2d)
    pub size_deep: f32,  // 피격 범위(z축)
}

impl Enemy {
    pub fn dead(&self, commands: &mut Commands, entity: Entity) {
        // 여기에 죽음 이펙트 추가
        commands.entity(entity).despawn();
    }
}

#[derive(Resource)]
pub struct EnemyMeshes {
    normal: Vec<Handle<Mesh>>,
    rare: Vec<Handle<Mesh>>,
    unique: Vec<Handle<Mesh>>,
    boss: Vec<Handle<Mesh>>,
}
impl EnemyMeshes {
    pub fn setup(meshes: &mut ResMut<Assets<Mesh>>) -> Self {
        let mut temp = EnemyMeshes {
            normal: Vec::with_capacity(1),
            rare: Vec::with_capacity(1),
            unique: Vec::with_capacity(1),
            boss: Vec::with_capacity(1),
        };

        let parts: Vec<(f32, f32, f32, f32)> = vec![(0., 0., 5., 5.)];
        let mesh = make_rect_mesh(&parts, Color::WHITE);
        let handle = meshes.add(mesh);
        temp.normal.push(handle.clone());

        temp
    }
}

pub fn enemy_mesh_setup_system(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.insert_resource(EnemyMeshes::setup(&mut meshes));
}

#[derive(Resource)]
pub struct EnemyWave {
    level: usize,
}
