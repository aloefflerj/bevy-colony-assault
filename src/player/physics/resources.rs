use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct PlayerJumpProperties {
    jumped_from_y_pos: f32,
}

impl Default for PlayerJumpProperties {
    fn default() -> Self {
        Self {
            jumped_from_y_pos: 0.,
        }
    }
}

impl PlayerJumpProperties {
    pub fn set_jumped_from(&mut self, y: f32) {
        self.jumped_from_y_pos = y;
    }

    pub fn jumped_from(&self) -> f32 {
        self.jumped_from_y_pos
    }

    pub fn jump_diff(&self, current_y_pos: f32) -> f32 {
        self.jumped_from_y_pos.abs() + current_y_pos
    }
}
