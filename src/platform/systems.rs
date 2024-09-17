use crate::collider::components::*;
use crate::config::*;
use crate::platform::components::*;
use bevy::prelude::*;

pub const PLATFORM_SPRITE_WIDTH: u32 = 16;
pub const PLATFORM_SPRITE_HEIGHT: u32 = 16;
const PLATFORM_SPRITE_ROWS: u32 = 4;
const PLATFORM_SPRITE_COLUMNS: u32 = 4;

pub fn spawn_platform1(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("tiles/tiles1.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(PLATFORM_SPRITE_WIDTH, PLATFORM_SPRITE_HEIGHT),
        PLATFORM_SPRITE_COLUMNS,
        PLATFORM_SPRITE_ROWS,
        None,
        None,
    );
    let texture_atlas_layouts = texture_atlas_layouts.add(layout);

    commands.spawn((
        Platform,
        SpriteBundle {
            texture,
            transform: Transform::from_xyz(
                -ORTOGRAPHIC_WINDOW_WIDTH / 2. + (PLATFORM_SPRITE_WIDTH as f32 * 9.),
                -ORTOGRAPHIC_WINDOW_HEIGHT / 2. + (PLATFORM_SPRITE_WIDTH as f32 * 2.),
                0.,
            ),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layouts,
            index: 2,
        },
        ColliderConfig {
            size: Vec2::new(PLATFORM_SPRITE_WIDTH as f32, PLATFORM_SPRITE_HEIGHT as f32),
            collider_type: ColliderType::Solid,
        },
    ));
}

pub fn spawn_platform2(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("tiles/tiles1.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(PLATFORM_SPRITE_WIDTH, PLATFORM_SPRITE_HEIGHT),
        PLATFORM_SPRITE_COLUMNS,
        PLATFORM_SPRITE_ROWS,
        None,
        None,
    );
    let texture_atlas_layouts = texture_atlas_layouts.add(layout);

    commands.spawn((
        Platform,
        SpriteBundle {
            texture,
            transform: Transform::from_xyz(
                -ORTOGRAPHIC_WINDOW_WIDTH / 2. + (PLATFORM_SPRITE_WIDTH as f32 * 10.),
                -ORTOGRAPHIC_WINDOW_HEIGHT / 2. + (PLATFORM_SPRITE_WIDTH as f32 * 2.),
                0.,
            ),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layouts,
            index: 3,
        },
        ColliderConfig {
            size: Vec2::new(PLATFORM_SPRITE_WIDTH as f32, PLATFORM_SPRITE_HEIGHT as f32),
            collider_type: ColliderType::Solid,
        },
    ));
}

pub fn spawn_platform3(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("tiles/tiles1.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(PLATFORM_SPRITE_WIDTH, PLATFORM_SPRITE_HEIGHT),
        PLATFORM_SPRITE_COLUMNS,
        PLATFORM_SPRITE_ROWS,
        None,
        None,
    );
    let texture_atlas_layouts = texture_atlas_layouts.add(layout);

    commands.spawn((
        Platform,
        SpriteBundle {
            texture,
            transform: Transform::from_xyz(
                -ORTOGRAPHIC_WINDOW_WIDTH / 2. + (PLATFORM_SPRITE_WIDTH as f32 * 11.),
                -ORTOGRAPHIC_WINDOW_HEIGHT / 2. + (PLATFORM_SPRITE_WIDTH as f32 * 5.),
                0.,
            ),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layouts,
            index: 2,
        },
        ColliderConfig {
            size: Vec2::new(PLATFORM_SPRITE_WIDTH as f32, PLATFORM_SPRITE_HEIGHT as f32),
            collider_type: ColliderType::Solid,
        },
    ));
}

pub fn spawn_platform4(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("tiles/tiles1.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(PLATFORM_SPRITE_WIDTH, PLATFORM_SPRITE_HEIGHT),
        PLATFORM_SPRITE_COLUMNS,
        PLATFORM_SPRITE_ROWS,
        None,
        None,
    );
    let texture_atlas_layouts = texture_atlas_layouts.add(layout);

    commands.spawn((
        Platform,
        SpriteBundle {
            texture,
            transform: Transform::from_xyz(
                -ORTOGRAPHIC_WINDOW_WIDTH / 2. + (PLATFORM_SPRITE_WIDTH as f32 * 12.),
                -ORTOGRAPHIC_WINDOW_HEIGHT / 2. + (PLATFORM_SPRITE_WIDTH as f32 * 5.),
                0.,
            ),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layouts,
            index: 3,
        },
        ColliderConfig {
            size: Vec2::new(PLATFORM_SPRITE_WIDTH as f32, PLATFORM_SPRITE_HEIGHT as f32),
            collider_type: ColliderType::Solid,
        },
    ));
}
