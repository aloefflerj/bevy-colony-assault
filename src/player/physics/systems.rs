use std::f32::INFINITY;

use crate::platform::components::*;
use crate::platform::systems::PLATFORM_HEIGHT;
use crate::player::components::*;
use crate::player::physics::resources::*;
use crate::player::physics::*;
use crate::player::PLAYER_SPRITE_HEIGHT;
use crate::GRAVITY_MULTIPLIER;
use bevy::prelude::*;
use components::PlayerVelocity;

pub fn move_x(
    key_board_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut PlayerVelocity), With<Player>>,
) {
    if let Ok((mut player, mut player_velocity)) = query.get_single_mut() {
        let mut movement = 0.;

        if key_board_input.pressed(KeyCode::ArrowRight) {
            movement += (time.delta_seconds() * PLAYER_X_SPEED).round();
        }

        if key_board_input.pressed(KeyCode::ArrowLeft) {
            movement += (time.delta_seconds() * PLAYER_X_SPEED).round() * -1.;
        }
        let initial_position = player.translation.x;
        player.translation.x += movement;

        let final_position = player.translation.x;
        let delta_position = final_position - initial_position;
        let average_speed = time.delta_seconds() / delta_position;

        player_velocity.velocity.x = if average_speed == INFINITY {
            0.
        } else {
            average_speed
        };
    }
}

pub fn jump(
    input: Res<ButtonInput<KeyCode>>,
    query: Query<&Transform, With<Player>>,
    mut player_jump_properties: ResMut<PlayerJumpProperties>,
    mut grounded_next_state: ResMut<NextState<PlayerGroundState>>,
) {
    if query.is_empty() {
        return;
    }

    if let Ok(player) = query.get_single() {
        if input.just_pressed(KeyCode::ArrowUp) {
            player_jump_properties.set_jumped_from(player.translation.y);
            grounded_next_state.set(PlayerGroundState::Rising);
        }
    }
}

pub fn rise(
    time: Res<Time>,
    player_jump_properties: Res<PlayerJumpProperties>,
    mut query: Query<(&mut Transform, &mut PlayerVelocity), With<Player>>,
    mut grounded_next_state: ResMut<NextState<PlayerGroundState>>,
) {
    if query.is_empty() {
        return;
    }

    if let Ok((mut player, mut player_velocity)) = query.get_single_mut() {
        if player_jump_properties.jump_diff(player.translation.y) >= PLAYER_MAX_JUMP_HEIGHT {
            grounded_next_state.set(PlayerGroundState::Falling);
        }

        let acceleration = (PLAYER_JUMP_SPEED - player_velocity.velocity.y) / time.delta_seconds();

        player.translation.y = player.translation.y
            + player_velocity.velocity.y * time.delta_seconds()
            + (1.5 * acceleration * (time.delta_seconds().powi(2)));

        player_velocity.velocity.y += acceleration * time.delta_seconds();
    }
}

pub fn apply_gravity(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut PlayerVelocity), With<Player>>,
) {
    if query.is_empty() {
        return;
    }

    if let Ok((mut player, mut player_velocity)) = query.get_single_mut() {
        player.translation.y = player.translation.y
            + player_velocity.velocity.y * time.delta_seconds()
            + (1.5 * GRAVITY_MULTIPLIER * (time.delta_seconds().powi(2)));

        player_velocity.velocity.y += GRAVITY_MULTIPLIER * time.delta_seconds();
    }
}

pub fn handle_ground_touch(
    mut player_query: Query<&mut Transform, With<Player>>,
    platform_query: Query<&Transform, (With<Platform>, Without<Player>)>,
    mut grounded_next_state: ResMut<NextState<PlayerGroundState>>,
) {
    if player_query.is_empty() || platform_query.is_empty() {
        return;
    }

    if let Err(_) = player_query.get_single_mut() {
        return;
    }

    let mut player = player_query.get_single_mut().unwrap();
    for platform in platform_query.iter() {
        let y_distance = player.translation.y - platform.translation.y;
        let player_radius = PLAYER_SPRITE_HEIGHT as f32 / 2.;
        let platform_radius = PLATFORM_HEIGHT / 2.;

        if y_distance <= player_radius + platform_radius {
            player.translation.y = platform.translation.y + player_radius + platform_radius;
            grounded_next_state.set(PlayerGroundState::Grounded);
        }
    }
}

pub fn db(query: Query<&PlayerVelocity>) {
    if query.is_empty() {
        return;
    }

    dbg!(query.get_single().unwrap());
}
