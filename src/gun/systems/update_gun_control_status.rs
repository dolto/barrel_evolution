use bevy::{input::touch::Touch, prelude::*};

use crate::gun::gun::GunControlStatus;

pub fn update_gun_control_status(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    mut status: ResMut<GunControlStatus>,
    mut last_touch: Local<Option<Touch>>, // 마지막 활성 터치 저장
) {
    // firing 토글 (마우스/터치 둘 다에서 발생)
    if mouse_buttons.just_pressed(MouseButton::Left) || touches.any_just_pressed() {
        status.firing = !status.firing;
    }

    // aiming: 마우스 누르거나 터치가 있으면 true
    let mut aiming = mouse_buttons.pressed(MouseButton::Left);

    if touches.iter().next().is_some() {
        aiming = true;

        // 마지막 눌린 터치를 저장
        if let Some(finger) = touches.iter().max_by_key(|f| f.id()) {
            *last_touch = Some(finger.clone());
        }
    } else {
        *last_touch = None;
    }

    status.aiming = aiming;
}
