use bevy::prelude::*;

use crate::player::animation::components::*;

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
            idle_animation: PlayerIdleAnimation {
                first_frame: 0,
                last_frame: 2,
            },
            run_animation: PlayerRunAnimation {
                first_frame: 4,
                last_frame: 7,
            },
            attack_animation: PlayerAttackAnimation {
                first_frame: 8,
                last_frame: 11,
            },
            jump_animation: PlayerJumpAnimation {
                first_frame: 12,
                last_frame: 13,
            },
            fall_animation: PlayerFallAnimation {
                first_frame: 14,
                last_frame: 15,
            },
        }
    }

    pub fn initial_frame() -> usize {
        0
    }
}
