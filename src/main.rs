mod animation;
mod camera;
mod config;
mod player;

use bevy::prelude::*;
use bevy_rapier2d::{plugin::*, render::RapierDebugRenderPlugin};
use camera::CameraPlugin;
use config::*;
use player::PlayerPlugin;

fn main() {
    let default_plugin_options = DefaultPluginOptions::new();
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(default_plugin_options.image_plugin)
                .set(default_plugin_options.window_plugin),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
