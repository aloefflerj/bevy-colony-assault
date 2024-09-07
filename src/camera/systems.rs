use bevy::prelude::*;

use crate::ORTOGRAPHIC_PROJECTION_SCALE;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: ORTOGRAPHIC_PROJECTION_SCALE,
            far: 1000.0,
            near: -1000.0,
            ..default()
        },
        ..default()
    });
}
