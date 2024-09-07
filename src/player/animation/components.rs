use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerIdleAnimation {
    pub first_frame: usize,
    pub last_frame: usize,
}

impl PlayerIdleAnimation {
    pub fn default_frames_qty() -> usize {
        3
    }
}

#[derive(Component)]
pub struct PlayerRunAnimation {
    pub first_frame: usize,
    pub last_frame: usize,
}

#[derive(Component)]
pub struct PlayerAttackAnimation {
    pub first_frame: usize,
    pub last_frame: usize,
}

#[derive(Component)]
pub struct PlayerJumpAnimation {
    pub first_frame: usize,
    pub last_frame: usize,
}

#[derive(Component)]
pub struct PlayerFallAnimation {
    pub first_frame: usize,
    pub last_frame: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct PlayerAnimationTimer(pub Timer);
