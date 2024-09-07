pub mod bundles;
pub mod components;
pub mod systems;

use crate::player::animation::systems::*;
use bevy::prelude::*;

pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_idle);
    }
}
