mod animation;
mod components;
mod physics;
pub mod systems;

use animation::PlayerAnimationPlugin;
use bevy::prelude::*;
use physics::PlayerPhysicsPlugin;
use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_plugins((PlayerPhysicsPlugin, PlayerAnimationPlugin));
    }
}
