use crate::animation::animator::*;
use crate::player::animation::components::*;
use crate::player::components::*;
use bevy::prelude::*;
use bevy::transform::commands;
use bevy_rapier2d::prelude::*;

use super::PlayerAnimationState;

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

pub fn handle_animation_state(
    mut query: Query<
        (
            &KinematicCharacterControllerOutput,
            &mut PlayerAnimationTimer,
        ),
        With<Player>,
    >,
    animation_state: Res<State<PlayerAnimationState>>,
    mut animation_next_state: ResMut<NextState<PlayerAnimationState>>,
) {
    if query.is_empty() {
        return;
    }

    let (kinematic_output, mut timer) = query.single_mut();
    if kinematic_output.desired_translation.x == 0. {
        let animation = PlayerIdleAnimation::default();
        let frames_qty = animation.frames_qty() as f32;

        let animation_timer = PlayerAnimationTimer::new(frames_qty, TimerMode::Repeating);
        timer.set_duration(animation_timer.duration());
        if *animation_state != PlayerAnimationState::Idle {
            animation_next_state.set(PlayerAnimationState::Idle);
        }
    } else {
        let animation = PlayerRunAnimation::default();
        let frames_qty = animation.frames_qty() as f32;

        let animation_timer = PlayerAnimationTimer::new(frames_qty, TimerMode::Repeating);
        timer.set_duration(animation_timer.duration());
        if *animation_state != PlayerAnimationState::Run {
            animation_next_state.set(PlayerAnimationState::Run);
        }
    }
}

pub fn reset_animation(
    animation_state: Res<State<PlayerAnimationState>>,
    mut query: Query<(&PlayerIdleAnimation, &PlayerRunAnimation, &mut TextureAtlas), With<Player>>,
) {
    if query.is_empty() {
        return;
    }

    let (idle, run, mut texture_atlas) = query.single_mut();

    let atlas_reset_animation_index = match animation_state.clone() {
        PlayerAnimationState::Idle => idle.first_frame,
        PlayerAnimationState::Run => run.first_frame,
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
