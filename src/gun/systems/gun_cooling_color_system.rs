use bevy::prelude::*;

use crate::gun::{barrel::BarrelSprite, gun::Gun};

pub fn gun_cooling_color_system(
    gun: Single<&Gun>,
    barrel_sprite: Query<&BarrelSprite>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for barrel_sprite in barrel_sprite {
        let barrel = &gun.barrels[barrel_sprite.index];

        let hot = (barrel.hp / barrel.max_hp).max(0.4667);
        let g_b = 0.4667 * (1.0 - (hot - 0.4467)); // hot 높을수록 녹청 감소
        if let Some(material) = materials.get_mut(&barrel_sprite.material) {
            material.color = Color::srgb(hot, g_b, g_b);
        }
    }
}
