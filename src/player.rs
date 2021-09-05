use raylib::prelude::*;

use crate::input::Input;

const WALK_SPEED: f32 = 300.0;
const SPRINT_SPEED: f32 = 600.0;

pub struct Player {
    pub pos: Vector2,
    move_speed: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: Vector2::zero(),
            move_speed: WALK_SPEED,
        }
    }

    pub fn update(&mut self, delta_time: f32, input: Input) {
        let dir = Vector2::new(input.input_h, -input.input_v);
        if input.sprint_key {
            self.move_speed = SPRINT_SPEED;
        } else {
            self.move_speed = WALK_SPEED;
        }
        if dir.length() > 0.0 {
            self.pos += dir.normalized() * delta_time * self.move_speed;
        }
    }
}
