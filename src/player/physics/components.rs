use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct PlayerVelocity {
    pub velocity: Vec2,
}

impl PlayerVelocity {
    pub fn default() -> Self {
        Self {
            velocity: Vec2::new(0., 0.),
        }
    }
}
