pub mod components;
mod systems;

use crate::collider::systems::*;
use crate::sets::*;
use bevy::prelude::*;

pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_colliders);
        app.add_systems(
            Update,
            stick_colliders_to_entities.in_set(HandleColliderPhysicsForce),
        );
    }
}
