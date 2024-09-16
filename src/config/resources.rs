use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct DebugOptions {
    pub collider: bool,
}

impl Default for DebugOptions {
    fn default() -> Self {
        Self { collider: true }
    }
}
