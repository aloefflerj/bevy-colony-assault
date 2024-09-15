use bevy::{prelude::*, window::WindowResolution};

pub const WINDOW_WIDTH: f32 = 1920.0;
pub const WINDOW_HEIGHT: f32 = 1080.0;

pub const ORTOGRAPHIC_WINDOW_WIDTH: f32 = WINDOW_WIDTH * ORTOGRAPHIC_PROJECTION_SCALE;
pub const ORTOGRAPHIC_WINDOW_HEIGHT: f32 = WINDOW_HEIGHT * ORTOGRAPHIC_PROJECTION_SCALE;

pub const WINDOW_LEFT_X: f32 = ORTOGRAPHIC_WINDOW_WIDTH / -2.0;
pub const WINDOW_BOTTOM_Y: f32 = ORTOGRAPHIC_WINDOW_HEIGHT / -2.0;

pub const ORTOGRAPHIC_PROJECTION_SCALE: f32 = 0.166666667;

pub const GRAVITY_MULTIPLIER: f32 = -600.;

pub struct DefaultPluginOptions {
    pub window_plugin: WindowPlugin,
    pub image_plugin: ImagePlugin,
}

impl DefaultPluginOptions {
    pub fn new() -> Self {
        Self {
            window_plugin: WindowPlugin {
                primary_window: Some(Window {
                    title: "Ant Colony".to_string(),
                    resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                    resizable: true,
                    ..default()
                }),
                ..default()
            },
            image_plugin: ImagePlugin::default_nearest(),
        }
    }
}
