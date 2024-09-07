mod systems;

use crate::player::physics::systems::*;
use bevy::prelude::*;

pub struct PlayerPhysicsPlugin;

impl Plugin for PlayerPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_x, fall));
    }
}
