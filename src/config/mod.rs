use bevy::{prelude::*, window::WindowResolution};

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;

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
