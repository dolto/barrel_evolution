use bevy::prelude::*;

use crate::gun::barrel::Barrel;

#[derive(Component)]
pub struct GunSpin;

#[derive(Component)]
pub struct Gun {
    pub speed: f32,           // 회전속도
    pub barrels: Vec<Barrel>, // 총열 entity모음
    pub radius: f32,          // 반지름
    pub recoil_control: f32,  // 반동을 제어하는 정도
    pub aim_speed: f32,       // 에임 스피드
    pub heal: f32,            // 발사 멈출 시 hp 회복량
}

#[derive(Resource)]
pub struct GunControlStatus {
    pub aiming: bool, // 마우스를 누르고 있는 동안 true
    pub firing: bool, // 클릭 토글
    pub aim_position: Vec2,
    pub is_enemy_z: bool, // 조준하는 z축에 적이 있는경우 true
}

#[derive(Component)]
pub struct GunAimCircle;
