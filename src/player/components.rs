use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub enum PlayerDirection {
    Left,
    Right,
}
