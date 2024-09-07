use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::WINDOW_BOTTOM_Y;

const PLATFORM_COLOR: Color = Color::linear_rgb(0.29, 0.31, 0.41);

#[derive(Bundle)]
pub struct PlatformBundle {
    sprite_bundle: SpriteBundle,
    body: RigidBody,
    collider: Collider,
}

impl PlatformBundle {
    pub fn new(x: f32, scale: Vec3) -> Self {
        Self {
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
            body: RigidBody::Fixed,
            collider: Collider::cuboid(0.5, 0.5),
        }
    }
}
