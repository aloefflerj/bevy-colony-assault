use bevy::prelude::*;

#[derive(Component, PartialEq)]
pub enum ColliderType {
    Actor,
    Solid,
}

#[derive(Component)]
pub struct Actor;

#[derive(Component)]
pub struct Solid;

#[derive(Component)]
pub struct ColliderConfig {
    pub size: Vec2,
    pub collider_type: ColliderType,
}

#[derive(Component)]
pub struct ColliderObject {
    pub attached_to_entity_id: u32,
}
