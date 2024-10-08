use crate::config::*;
use crate::platform::components::*;
use crate::platform::systems::*;
use crate::player::components::*;
use crate::player::physics::components::*;
use crate::player::physics::resources::*;
use crate::player::physics::*;
use crate::player::systems::*;
use bevy::prelude::*;
use std::f32::INFINITY;

const PLATFORM_COLLISION_TOLERANCE_MARGIN: f32 = 0.1;
const PLATFORM_COLLISION_HEIGHT: f32 = 5.;

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

pub fn handle_gravity(
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
    mut player_query: Query<(&mut Transform, &mut PlayerVelocity), With<Player>>,
    platform_query: Query<&Transform, (With<Platform>, Without<Player>)>,
    mut grounded_next_state: ResMut<NextState<PlayerGroundState>>,
) {
    if player_query.is_empty() || platform_query.is_empty() {
        return;
    }

    if let Err(_) = player_query.get_single_mut() {
        return;
    }

    let (mut player, mut player_velocity) = player_query.single_mut();
    let mut on_platform_translation_y: Option<f32> = None;

    let player_y_radius = PLAYER_SPRITE_HEIGHT as f32 / 2.;
    let player_x_radius = PLAYER_SPRITE_WIDTH as f32 / 2.;

    let platform_y_radius = PLATFORM_SPRITE_HEIGHT as f32 / 2.;
    let platform_x_radius = PLATFORM_SPRITE_WIDTH as f32 / 2.;

    for platform in platform_query.iter() {
        let y_distance = player.translation.y - platform.translation.y;
        let x_distance = player.translation.x - platform.translation.x;
        let x_distance = x_distance.abs();
        let top_platform_limit =
            player_y_radius + platform_y_radius + PLATFORM_COLLISION_TOLERANCE_MARGIN;

        let bottom_platform_limit = player_y_radius + platform_y_radius - PLATFORM_COLLISION_HEIGHT;

        if (y_distance <= top_platform_limit && y_distance >= bottom_platform_limit)
            && x_distance < player_x_radius + platform_x_radius
        {
            on_platform_translation_y = Some(platform.translation.y);
            break;
        }
    }

    match on_platform_translation_y {
        Some(platform_translation_y) => {
            player.translation.y = platform_translation_y + player_y_radius + platform_y_radius;
            grounded_next_state.set(PlayerGroundState::Grounded);
            player_velocity.velocity.y = 0.;
        }
        None => {
            grounded_next_state.set(PlayerGroundState::Falling);
        }
    }
}
