pub mod components;
pub mod systems;

use crate::platform::systems::*;
use bevy::prelude::*;

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreStartup,
            (
                spawn_platform1,
                spawn_platform2,
                spawn_platform3,
                spawn_platform4,
            ),
        );
    }
}
