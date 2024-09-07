mod animation;
mod components;
mod systems;

use animation::PlayerAnimationPlugin;
use bevy::prelude::*;
use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerAnimationPlugin);
        app.add_systems(Startup, spawn_player);
    }
}
