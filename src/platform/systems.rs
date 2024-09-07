use bevy::prelude::*;

use crate::{ORTOGRAPHIC_WINDOW_HEIGHT, ORTOGRAPHIC_WINDOW_WIDTH};

use super::bundles::PlatformBundle;

pub fn spawn_platform(mut commands: Commands) {
    commands.spawn(PlatformBundle::new(
        0.,
        Vec3::new(
            ORTOGRAPHIC_WINDOW_WIDTH as f32,
            ORTOGRAPHIC_WINDOW_HEIGHT as f32 / 5.,
            0.,
        ),
    ));
}
