use bevy::prelude::*;

pub fn animate_in_loop(
    time: &Res<Time>,
    first_frame: usize,
    last_frame: usize,
    timer: &mut Timer,
    texture_atlas: &mut TextureAtlas,
) {
    timer.tick(time.delta());
    if !timer.just_finished() {
        return;
    }

    texture_atlas.index = if texture_atlas.index >= last_frame {
        first_frame
    } else {
        texture_atlas.index + 1
    };
}

pub fn animate_once(
    time: &Res<Time>,
    last_frame: usize,
    timer: &mut Timer,
    texture_atlas: &mut TextureAtlas,
) {
    timer.tick(time.delta());
    if !timer.just_finished() {
        return;
    }

    texture_atlas.index = if texture_atlas.index == last_frame {
        last_frame
    } else {
        texture_atlas.index + 1
    };
}
