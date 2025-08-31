use bevy::prelude::*;

use crate::{
    gun::gun::{Gun, GunControlStatus},
    util::make_rect_mesh,
};

pub struct Barrel {
    pub power: f32,      // 반동
    pub hp: f32,         // 내구도 높을수록 안좋음
    pub hp_step: f32,    // 발사마다 내구도 소모량
    pub max_hp: f32,     // 최대 내구도
    pub reload: bool,    // 발사가능
    pub is_broken: bool, // max_hp까지 내구도가 올라가면 true, hp가 0이 될 때 까지 발사 못함
    pub bullet: Bullet,
}

impl Barrel {
    pub fn fire(
        &self,
        gun: &Gun,
        gun_control_status: &mut ResMut<GunControlStatus>,
        g_global: &GlobalTransform,
    ) -> Bullet {
        let up = g_global.up();
        let recoil = (Vec2::new(fastrand::f32() * 2. - 1., fastrand::f32() * 2. - 1.).normalize()
            * self.power
            * (1. + self.hp / self.max_hp))
            * (1. - gun.recoil_control);
        gun_control_status.aim_position += recoil;
        self.bullet.clone().with_up(up)
    }
}

#[derive(Component)]
pub struct BarrelSprite {
    pub index: usize,
    pub material: Handle<ColorMaterial>,
}

#[derive(Component, Clone)]
pub struct Bullet {
    pub up: Dir3,
    pub damage: f32,
    pub speed: f32,
    pub bullet_effect: Vec<BulletEffect>,
    pub is_enemy: bool,
    pub size: f32, // 피격 거리
}

impl Bullet {
    pub fn with_up(mut self, up: Dir3) -> Self {
        self.up = up;
        self
    }

    pub fn hit(&self, commands: &mut Commands, entity: Entity) {
        // 여기에 hit효과 추가
        commands.entity(entity).despawn();
    }
}

#[derive(Clone)]
pub enum BulletEffect {
    Particle,
    Color,
    SpeedChange,
    Rotate,
}

#[derive(Resource)]
pub struct BarrelModel {
    pub prototype: Handle<Mesh>,
    // 모델 추가 예정
}

impl BarrelModel {
    fn setup(meshes: &mut ResMut<Assets<Mesh>>) -> Self {
        let parts: Vec<(f32, f32, f32, f32)> = vec![(0., 0., 2., 20.)];
        let prototype = meshes.add(make_rect_mesh(&parts, Color::WHITE));
        BarrelModel { prototype }
    }
}

pub fn barrel_model_setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.insert_resource(BarrelModel::setup(&mut meshes));
}
