use crate::animation::animator::*;
use crate::player::animation::components::*;
use crate::player::animation::*;
use crate::player::components::*;
use crate::player::physics::components::PlayerVelocity;
use bevy::prelude::*;

pub fn animate_idle(
    time: Res<Time>,
    mut query: Query<
        (
            &PlayerIdleAnimation,
            &mut PlayerAnimationTimer,
            &mut TextureAtlas,
        ),
        With<Player>,
    >,
) {
    for (idle_animation, mut timer, mut texture_atlas) in &mut query {
        animate_in_loop(
            &time,
            idle_animation.first_frame,
            idle_animation.last_frame,
            &mut timer,
            &mut texture_atlas,
        );
    }
}

pub fn animate_run(
    time: Res<Time>,
    mut query: Query<
        (
            &PlayerRunAnimation,
            &mut PlayerAnimationTimer,
            &mut TextureAtlas,
        ),
        With<Player>,
    >,
) {
    for (run_animation, mut timer, mut texture_atlas) in &mut query {
        animate_in_loop(
            &time,
            run_animation.first_frame,
            run_animation.last_frame,
            &mut timer,
            &mut texture_atlas,
        );
    }
}

pub fn animate_jump(
    time: Res<Time>,
    mut query: Query<
        (
            &PlayerJumpAnimation,
            &mut PlayerAnimationTimer,
            &mut TextureAtlas,
        ),
        With<Player>,
    >,
) {
    for (jump_animation, mut timer, mut texture_atlas) in &mut query {
        animate_in_loop(
            &time,
            jump_animation.first_frame,
            jump_animation.last_frame,
            &mut timer,
            &mut texture_atlas,
        );
    }
}

pub fn animate_fall(
    time: Res<Time>,
    mut query: Query<
        (
            &PlayerFallAnimation,
            &mut PlayerAnimationTimer,
            &mut TextureAtlas,
        ),
        With<Player>,
    >,
) {
    for (fall_animation, mut timer, mut texture_atlas) in &mut query {
        animate_in_loop(
            &time,
            fall_animation.first_frame,
            fall_animation.last_frame,
            &mut timer,
            &mut texture_atlas,
        );
    }
}

pub fn handle_animation_state(
    mut query: Query<(&PlayerVelocity, &mut PlayerAnimationTimer), With<Player>>,
    animation_state: Res<State<PlayerAnimationState>>,
    animation_next_state: ResMut<NextState<PlayerAnimationState>>,
) {
    if query.is_empty() {
        return;
    }

    let (player_velocity, mut timer) = query.single_mut();

    if player_velocity.velocity.y == 0. && player_velocity.velocity.x == 0. {
        let animation = PlayerIdleAnimation::default();
        let new_animation_state = PlayerAnimationState::Idle;

        update_animation_state(
            &animation.frames_qty(),
            TimerMode::Repeating,
            1.,
            &mut timer,
            animation_state,
            new_animation_state,
            animation_next_state,
        );
    } else if player_velocity.velocity.y == 0. && player_velocity.velocity.x != 0. {
        let animation = PlayerRunAnimation::default();
        let new_animation_state = PlayerAnimationState::Run;

        update_animation_state(
            &animation.frames_qty(),
            TimerMode::Repeating,
            1.5,
            &mut timer,
            animation_state,
            new_animation_state,
            animation_next_state,
        );
    } else if player_velocity.velocity.y > 0. {
        let animation = PlayerJumpAnimation::default();
        let new_animation_state = PlayerAnimationState::Jump;

        update_animation_state(
            &animation.frames_qty(),
            TimerMode::Repeating,
            1.,
            &mut timer,
            animation_state,
            new_animation_state,
            animation_next_state,
        );
    } else if player_velocity.velocity.y < 0. {
        let animation = PlayerFallAnimation::default();
        let new_animation_state = PlayerAnimationState::Fall;

        update_animation_state(
            &animation.frames_qty(),
            TimerMode::Repeating,
            1.,
            &mut timer,
            animation_state,
            new_animation_state,
            animation_next_state,
        );
    }
}

fn update_animation_state(
    frames_qty: &usize,
    timer_mode: TimerMode,
    fps_multiplier: f32,
    timer: &mut PlayerAnimationTimer,
    animation_state: Res<State<PlayerAnimationState>>,
    new_animation_state: PlayerAnimationState,
    mut animation_next_state: ResMut<NextState<PlayerAnimationState>>,
) {
    let animation_timer = PlayerAnimationTimer::new(*frames_qty as f32, timer_mode, fps_multiplier);
    timer.set_duration(animation_timer.duration());
    if *animation_state != new_animation_state {
        animation_next_state.set(new_animation_state);
    }
}

pub fn handle_direction_state(
    mut commands: Commands,
    query: Query<(Entity, &PlayerVelocity), With<Player>>,
) {
    if query.is_empty() {
        return;
    }

    let (player, player_velocity) = query.single();

    if player_velocity.velocity.x > 0. {
        commands.entity(player).insert(PlayerDirection::Right);
    } else if player_velocity.velocity.x < 0. {
        commands.entity(player).insert(PlayerDirection::Left);
    }
}

pub fn reset_animation(
    animation_state: Res<State<PlayerAnimationState>>,
    mut query: Query<
        (
            &PlayerIdleAnimation,
            &PlayerRunAnimation,
            &PlayerJumpAnimation,
            &PlayerFallAnimation,
            &mut TextureAtlas,
        ),
        With<Player>,
    >,
) {
    if query.is_empty() {
        return;
    }

    let (idle, run, jump, fall, mut texture_atlas) = query.single_mut();

    let atlas_reset_animation_index = match animation_state.clone() {
        PlayerAnimationState::Idle => idle.first_frame,
        PlayerAnimationState::Run => run.first_frame,
        PlayerAnimationState::Jump => jump.first_frame,
        PlayerAnimationState::Fall => fall.first_frame,
        _ => 0,
    };

    texture_atlas.index = atlas_reset_animation_index;
}

pub fn reset_timer(mut query: Query<&mut PlayerAnimationTimer, With<Player>>) {
    if query.is_empty() {
        return;
    }

    let mut animation_timer = query.single_mut();
    animation_timer.reset();
}

pub fn update_sprite_direction(mut query: Query<(&mut Sprite, &PlayerDirection)>) {
    if query.is_empty() {
        return;
    }

    let (mut sprite, direction) = query.single_mut();
    match direction {
        PlayerDirection::Right => sprite.flip_x = false,
        PlayerDirection::Left => sprite.flip_x = true,
    };
}
