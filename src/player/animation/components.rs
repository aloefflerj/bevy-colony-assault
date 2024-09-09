use std::time::Duration;

use bevy::prelude::*;

use crate::animation::DEFAULT_ANIMATION_FPS;

#[derive(Component)]
pub struct PlayerIdleAnimation {
    pub first_frame: usize,
    pub last_frame: usize,
}

impl PlayerIdleAnimation {
    pub fn new(first_frame: usize, last_frame: usize) -> Self {
        Self {
            first_frame,
            last_frame,
        }
    }

    pub fn default() -> Self {
        Self {
            first_frame: 0,
            last_frame: 2,
        }
    }

    pub fn frames_qty(&self) -> usize {
        self.last_frame - self.first_frame + 1
    }

    pub fn default_frames_qty() -> usize {
        3
    }
}

#[derive(Component)]
pub struct PlayerRunAnimation {
    pub first_frame: usize,
    pub last_frame: usize,
}

impl PlayerRunAnimation {
    pub fn default() -> Self {
        Self {
            first_frame: 4,
            last_frame: 7,
        }
    }

    pub fn frames_qty(&self) -> usize {
        self.last_frame - self.first_frame + 1
    }
}
#[derive(Component)]
pub struct PlayerAttackAnimation {
    pub first_frame: usize,
    pub last_frame: usize,
}

impl PlayerAttackAnimation {
    pub fn default() -> Self {
        Self {
            first_frame: 8,
            last_frame: 11,
        }
    }

    pub fn frames_qty(&self) -> usize {
        self.last_frame - self.first_frame + 1
    }
}

#[derive(Component)]
pub struct PlayerJumpAnimation {
    pub first_frame: usize,
    pub last_frame: usize,
}

impl PlayerJumpAnimation {
    pub fn default() -> Self {
        Self {
            first_frame: 12,
            last_frame: 13,
        }
    }

    pub fn frames_qty(&self) -> usize {
        self.last_frame - self.first_frame + 1
    }
}

#[derive(Component)]
pub struct PlayerFallAnimation {
    pub first_frame: usize,
    pub last_frame: usize,
}

impl PlayerFallAnimation {
    pub fn default() -> Self {
        Self {
            first_frame: 14,
            last_frame: 15,
        }
    }

    pub fn frames_qty(&self) -> usize {
        self.last_frame - self.first_frame + 1
    }
}

#[derive(Component, Deref, DerefMut, Debug)]
pub struct PlayerAnimationTimer(pub Timer);

impl PlayerAnimationTimer {
    pub fn new(frames_qty: f32, timer_mode: TimerMode) -> Self {
        Self(Timer::new(
            Duration::from_secs_f32((1.0 / (DEFAULT_ANIMATION_FPS as f32)) * frames_qty),
            timer_mode,
        ))
    }

    pub fn default() -> Self {
        Self(Timer::new(
            Duration::from_secs_f32(
                (1.0 / (DEFAULT_ANIMATION_FPS as f32))
                    * PlayerIdleAnimation::default_frames_qty() as f32,
            ),
            TimerMode::Repeating,
        ))
    }
}
