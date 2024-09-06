use std::time::Duration;

use crate::sprite::components::*;
use bevy::prelude::*;

pub struct SpriteTileProperties {
    pub width: u32,
    pub height: u32,
    pub rows: u32,
    pub columns: u32,
}

#[derive(Bundle)]
pub struct AnimatedSprite {
    sprite_bundle: SpriteBundle,
    texture_atlas: TextureAtlas,
    animation_config: AnimationConfig,
    animation_timer: AnimationTimer,
}

impl AnimatedSprite {
    pub fn build(
        texture: Handle<Image>,
        animation_config: AnimationConfig,
        sprite_tile_properties: SpriteTileProperties,
        mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) -> Self {
        let sprite_bundle = SpriteBundle {
            transform: Transform::from_scale(Vec3::new(1., 1., 0.)),
            texture,
            ..default()
        };

        let texture_atlas_layout = TextureAtlasLayout::from_grid(
            UVec2::new(sprite_tile_properties.width, sprite_tile_properties.height),
            sprite_tile_properties.columns,
            sprite_tile_properties.rows,
            None,
            None,
        );

        let texture_atlas_layouts = texture_atlas_layouts.add(texture_atlas_layout);

        let AnimationConfig { fps, frames_qty } = animation_config;
        let timer = Timer::new(
            Duration::from_secs_f32((1.0 / (fps as f32)) * frames_qty as f32),
            TimerMode::Repeating,
        );
        let animation_timer = AnimationTimer(timer);

        let texture_atlas = TextureAtlas {
            index: animation_config.first_frame(),
            layout: texture_atlas_layouts,
        };

        Self {
            sprite_bundle,
            animation_config,
            texture_atlas,
            animation_timer,
        }
    }
}
