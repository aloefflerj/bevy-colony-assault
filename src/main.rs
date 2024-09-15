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
    let bg_color = Srgba::hex("14081a").unwrap();
    let bg_color = Color::srgba(bg_color.red, bg_color.green, bg_color.blue, bg_color.alpha);

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(default_plugin_options.image_plugin)
                .set(default_plugin_options.window_plugin),
        )
        .insert_resource(ClearColor(bg_color))
        .add_plugins(CameraPlugin)
        .add_plugins(PlatformPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
