use bevy::prelude::*;

use crate::gun::gun::Gun;

pub struct Barrel {
    pub power: f32,   // 반동
    pub hp: f32,      // 내구도 높을수록 안좋음
    pub max_hp: f32,  // 최대 내구도
    pub damage: f32,  // 공격력
    pub reload: bool, // 발사가능
    pub bullet_speed: f32,
}

impl Barrel {
    pub fn fire(
        &self,
        gun: &Gun,
        y: f32,
        g_trans: &mut Transform,
        g_global: &GlobalTransform,
    ) -> Bullet {
        let up = g_global.up();
        let recoil = (Vec3::new(fastrand::f32() * 2. - 1., 0., fastrand::f32() * 2. - 1.)
            .normalize()
            * self.power)
            * (1. - gun.recoil_control);
        g_trans.rotation *= Quat::from_euler(EulerRot::XYZ, recoil.x, recoil.y, recoil.z);
        Bullet {
            y,
            up,
            damage: self.damage,
            speed: self.bullet_speed,
        }
    }
}

#[derive(Component)]
pub struct BarrelSprite {
    pub index: usize,
}

#[derive(Component)]
pub struct Bullet {
    pub y: f32,
    pub up: Dir3,
    pub damage: f32,
    pub speed: f32,
}
