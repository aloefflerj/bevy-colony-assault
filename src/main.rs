mod animation;
mod camera;
mod config;
mod platform;
mod player;

use bevy::prelude::*;
use camera::CameraPlugin;
use config::*;
use platform::PlatformPlugin;
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
        .add_plugins(PlatformPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
