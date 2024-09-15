pub mod bundles;
pub mod components;
pub mod systems;

use crate::player::animation::systems::*;
use bevy::prelude::*;

pub struct PlayerAnimationPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct AnimationHandlerSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct AnimateSet;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum PlayerAnimationState {
    #[default]
    Idle,
    Run,
    Attack,
    Jump,
    Fall,
}

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PlayerAnimationState>();
        app.configure_sets(Update, AnimationHandlerSet.before(AnimateSet));
        app.add_systems(
            OnEnter(PlayerAnimationState::Idle),
            (
                reset_timer.in_set(AnimationHandlerSet),
                reset_animation.in_set(AnimationHandlerSet),
            ),
        );
        app.add_systems(
            OnEnter(PlayerAnimationState::Run),
            (
                reset_timer.in_set(AnimationHandlerSet),
                reset_animation.in_set(AnimationHandlerSet),
            ),
        );
        app.add_systems(
            Update,
            (
                handle_animation_state.in_set(AnimationHandlerSet),
                animate_idle
                    .run_if(in_state(PlayerAnimationState::Idle))
                    .in_set(AnimateSet),
                animate_run
                    .run_if(in_state(PlayerAnimationState::Run))
                    .in_set(AnimateSet),
            ),
        );
    }
}
