use bevy::prelude::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ApplyPhysicsForce;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct HandlePhysicsForce;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct HandleColliderPhysicsForce;
