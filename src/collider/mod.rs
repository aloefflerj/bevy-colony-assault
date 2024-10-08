pub mod components;
mod systems;

use crate::collider::systems::*;
use crate::sets::*;
use bevy::prelude::*;

pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                load_colliders.before(stick_solid_colliders_to_entities),
                stick_solid_colliders_to_entities,
            ),
        );
        app.add_systems(
            Update,
            stick_actor_colliders_to_entities.in_set(HandleColliderPhysicsForce),
        );
    }
}
