use crate::player::animation::components::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct PlayerAnimations {
    idle_animation: PlayerIdleAnimation,
    run_animation: PlayerRunAnimation,
    attack_animation: PlayerAttackAnimation,
    jump_animation: PlayerJumpAnimation,
    fall_animation: PlayerFallAnimation,
}

impl PlayerAnimations {
    pub fn build() -> Self {
        Self {
            idle_animation: PlayerIdleAnimation::default(),
            run_animation: PlayerRunAnimation::default(),
            attack_animation: PlayerAttackAnimation::default(),
            jump_animation: PlayerJumpAnimation::default(),
            fall_animation: PlayerFallAnimation::default(),
        }
    }

    pub fn params_to_tuple(
        &self,
    ) -> (
        &PlayerIdleAnimation,
        &PlayerRunAnimation,
        &PlayerAttackAnimation,
        &PlayerJumpAnimation,
        &PlayerFallAnimation,
    ) {
        (
            &self.idle_animation,
            &self.run_animation,
            &self.attack_animation,
            &self.jump_animation,
            &self.fall_animation,
        )
    }

    pub fn initial_frame() -> usize {
        0
    }
}
