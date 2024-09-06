use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AnimationConfig {
    pub frames_qty: usize,
    pub fps: u32,
}

impl AnimationConfig {
    pub fn first_frame(&self) -> usize {
        0
    }

    pub fn last_frame(&self) -> usize {
        self.frames_qty - 1
    }
}
