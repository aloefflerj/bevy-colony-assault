use crate::sprite::components::*;
use bevy::prelude::*;

pub fn animate_in_loop(
    time: &Res<Time>,
    animation_config: &AnimationConfig,
    animation_timer: &mut AnimationTimer,
    texture_atlas: &mut TextureAtlas,
) {
    animation_timer.tick(time.delta());
    if !animation_timer.just_finished() {
        return;
    }

    texture_atlas.index = if texture_atlas.index == animation_config.last_frame() {
        animation_config.first_frame()
    } else {
        texture_atlas.index + 1
    };
}
