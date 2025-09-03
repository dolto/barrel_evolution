use bevy::prelude::*;

#[derive(Component)]
pub struct DeadFlag(pub bool);

pub fn check_dead_flag_system(mut commands: Commands, dead_flags: Query<(Entity, &DeadFlag)>) {
    for (entity, dead_flag) in dead_flags {
        if dead_flag.0 {
            commands.entity(entity).despawn();
        }
    }
}
