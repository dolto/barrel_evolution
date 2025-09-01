use bevy::prelude::*;

use crate::{AimSprite, gun::gun::GunControlStatus};

// pub fn aim_circle_color_system(
//     mut enemy_sort: Local<bool>,
//     aim_color: Res<GunControlStatus>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     aim_sprite: Res<AimSprite>,
// ) {
//     if *enemy_sort != aim_color.is_enemy_z {
//         *enemy_sort = aim_color.is_enemy_z;

//         let aim_color;
//         if *enemy_sort {
//             aim_color = Color::srgba(0.1, 0.7, 0.1, 1.);
//         } else {
//             aim_color = Color::srgba(0.7, 0.1, 0.1, 1.);
//         }

//         materials.get_mut(&aim_sprite.material).unwrap().color = aim_color;
//     }
// }
