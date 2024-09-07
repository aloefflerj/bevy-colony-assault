use crate::animation::animator::*;
use crate::player::animation::components::*;
use bevy::prelude::*;

pub fn animate_idle(
    time: Res<Time>,
    mut query: Query<(
        &PlayerIdleAnimation,
        &mut PlayerAnimationTimer,
        &mut TextureAtlas,
    )>,
) {
    for (idle_animation, mut timer, mut texture_atlas) in &mut query {
        animate_in_loop(
            &time,
            idle_animation.first_frame,
            idle_animation.last_frame,
            &mut timer,
            &mut texture_atlas,
        )
    }
}
