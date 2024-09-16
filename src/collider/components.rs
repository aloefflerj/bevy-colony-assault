use bevy::prelude::*;

#[derive(Component)]
pub struct ColliderConfig {
    pub size: Vec2,
}

#[derive(Component)]
pub struct ColliderObject {
    pub attached_to_entity_id: u32,
}
