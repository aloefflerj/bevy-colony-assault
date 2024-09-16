use crate::collider::components::*;
use crate::player::animation::bundles::*;
use crate::player::animation::components::*;
use crate::player::components::*;
use crate::player::physics::components::*;
use bevy::prelude::*;

pub const PLAYER_SPRITE_WIDTH: u32 = 18;
pub const PLAYER_SPRITE_HEIGHT: u32 = 16;
const PLAYER_SPRITE_ROWS: u32 = 4;
const PLAYER_SPRITE_COLUMNS: u32 = 4;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Player);
    let texture = asset_server.load("player/player.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(PLAYER_SPRITE_WIDTH, PLAYER_SPRITE_HEIGHT),
        PLAYER_SPRITE_COLUMNS,
        PLAYER_SPRITE_ROWS,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        Player,
        SpriteBundle {
            texture,
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: PlayerAnimations::initial_frame(),
        },
        PlayerAnimations::build(),
        PlayerAnimationTimer::default(),
        PlayerDirection::Right,
        PlayerVelocity::default(),
        ColliderConfig {
            size: Vec2::new(16., 16.),
        },
    ));
}
