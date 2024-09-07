use bevy::prelude::*;
use bevy_rapier2d::prelude::KinematicCharacterController;

use crate::player::{self, components::*};

pub fn move_x(
    key_board_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut KinematicCharacterController, With<Player>>,
) {
    let mut player = query.single_mut();

    let mut movement = 0.;

    if key_board_input.pressed(KeyCode::ArrowRight) {
        movement += time.delta_seconds() * 400.;
    }

    if key_board_input.pressed(KeyCode::ArrowLeft) {
        movement += time.delta_seconds() * 400. * -1.;
    }

    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(movement, vec.y)),
        None => player.translation = Some(Vec2::new(movement, 0.)),
    }
}

pub fn fall(time: Res<Time>, mut query: Query<&mut KinematicCharacterController, With<Player>>) {
    let mut player = query.single_mut();
    let movement = time.delta().as_secs_f32() * (850. / 1.5) * -1.;

    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
        None => player.translation = Some(Vec2::new(0., movement)),
    }
}
