use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.166666667,
            far: 1000.0,
            near: -1000.0,
            ..default()
        },
        ..default()
    });
}
