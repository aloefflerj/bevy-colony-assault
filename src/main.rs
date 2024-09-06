mod animation;
mod camera;
mod config;
mod player;
mod sprite;

use bevy::prelude::*;
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
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
