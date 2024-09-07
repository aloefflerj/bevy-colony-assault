mod systems;

use crate::player::physics::systems::*;
use bevy::prelude::*;

const PLAYER_X_SPEED: f32 = 400.;
const PLAYER_Y_SPEED: f32 = 400.;
const PLAYER_FALL_SPEED: f32 = 850.;

pub struct PlayerPhysicsPlugin;

impl Plugin for PlayerPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_x, fall));
    }
}
