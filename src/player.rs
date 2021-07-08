use raylib::prelude::*;

use crate::Input;

const MOVE_SPEED: f32 = 1000.0;

pub struct Player {
    pub pos: Vector2,
} 

impl Player {
    pub fn new() -> Self {
        Self {
            pos: Vector2::zero()
        }
    }

    pub fn update(&mut self, delta_time: f32, input: Input) {
        let dir = Vector2::new(input.input_h, -input.input_v);
        if dir.length() > 0.0 {
            self.pos += dir.normalized() * delta_time * MOVE_SPEED;
        }
    }
}
