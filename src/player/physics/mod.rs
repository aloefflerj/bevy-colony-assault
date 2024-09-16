pub mod components;
mod resources;
mod systems;

use crate::player::physics::resources::*;
use crate::player::physics::systems::*;
use crate::sets::ApplyPhysicsForce;
use crate::sets::HandlePhysicsForce;
use bevy::prelude::*;

const PLAYER_X_SPEED: f32 = 150.;
const PLAYER_JUMP_SPEED: f32 = 250.;
const PLAYER_MAX_JUMP_HEIGHT: f32 = 1.;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum PlayerGroundState {
    #[default]
    Falling,
    Rising,
    Grounded,
}

pub struct PlayerPhysicsPlugin;

impl Plugin for PlayerPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PlayerGroundState>();
        app.init_resource::<PlayerJumpProperties>();
        app.add_systems(
            Update,
            (
                move_x.in_set(ApplyPhysicsForce),
                jump.run_if(in_state(PlayerGroundState::Grounded))
                    .in_set(ApplyPhysicsForce),
                rise.run_if(in_state(PlayerGroundState::Rising))
                    .in_set(ApplyPhysicsForce),
                handle_gravity
                    .run_if(
                        in_state(PlayerGroundState::Falling)
                            .or_else(in_state(PlayerGroundState::Rising)),
                    )
                    .in_set(HandlePhysicsForce),
                handle_ground_touch
                    .run_if(
                        in_state(PlayerGroundState::Falling)
                            .or_else(in_state(PlayerGroundState::Grounded)),
                    )
                    .in_set(HandlePhysicsForce),
            ),
        );
    }
}
