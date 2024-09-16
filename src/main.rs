mod animation;
mod camera;
mod collider;
mod config;
mod platform;
mod player;
mod sets;

use bevy::prelude::*;
use camera::CameraPlugin;
use collider::ColliderPlugin;
use config::*;
use platform::PlatformPlugin;
use player::PlayerPlugin;
use resources::DebugOptions;
use sets::*;

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
        .init_resource::<DebugOptions>()
        .insert_resource(ClearColor(bg_color))
        .configure_sets(
            Update,
            HandlePhysicsForce.before(HandleColliderPhysicsForce),
        )
        .configure_sets(Update, HandleColliderPhysicsForce.before(ApplyPhysicsForce))
        .add_plugins(CameraPlugin)
        .add_plugins(PlatformPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(ColliderPlugin)
        .run();
}
