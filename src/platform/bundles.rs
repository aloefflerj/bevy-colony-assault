use crate::config::*;
use crate::platform::components::Platform;
use bevy::prelude::*;

const PLATFORM_COLOR: Color = Color::linear_rgb(0.29, 0.31, 0.41);

#[derive(Bundle)]
pub struct PlatformBundle {
    platform: Platform,
    sprite_bundle: SpriteBundle,
}

impl PlatformBundle {
    pub fn new(x: f32, scale: Vec3) -> Self {
        Self {
            platform: Platform,
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: PLATFORM_COLOR,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(x, WINDOW_BOTTOM_Y + (scale.y / 2.), 0.),
                    scale,
                    ..default()
                },
                ..default()
            },
        }
    }
}
