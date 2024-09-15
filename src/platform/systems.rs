use bevy::prelude::*;

use crate::config::*;
use crate::platform::bundles::*;

pub const PLATFORM_HEIGHT: f32 = ORTOGRAPHIC_WINDOW_HEIGHT as f32 / 5.;

pub fn spawn_platform(mut commands: Commands) {
    commands.spawn(PlatformBundle::new(
        0.,
        Vec3::new(ORTOGRAPHIC_WINDOW_WIDTH as f32, PLATFORM_HEIGHT, 0.),
    ));
}
