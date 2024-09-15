mod bundles;
pub mod components;
pub mod systems;

use crate::platform::systems::*;
use bevy::prelude::*;

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_platform);
    }
}
