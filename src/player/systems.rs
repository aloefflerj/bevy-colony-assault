use crate::animation::*;
use crate::sprite::bundles::*;
use crate::sprite::components::*;
use bevy::prelude::*;

const SPRITE_TILE_WIDTH: u32 = 18;
const SPRITE_TILE_HEIGHT: u32 = 16;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(AnimatedSprite::build(
        asset_server.load("player/ant-idle.png"),
        AnimationConfig {
            frames_qty: 3,
            fps: 12,
        },
        SpriteTileProperties {
            width: SPRITE_TILE_WIDTH,
            height: SPRITE_TILE_HEIGHT,
            rows: 1,
            columns: 3,
        },
        texture_atlas_layouts,
    ));
}

pub fn animate_idle(
    time: Res<Time>,
    mut query: Query<(&AnimationConfig, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (animation_config, mut animation_timer, mut texture_atlas) in &mut query {
        animator::animate_in_loop(
            &time,
            animation_config,
            animation_timer.as_mut(),
            texture_atlas.as_mut(),
        );
    }
}
