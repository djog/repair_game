use raylib::prelude::*;
pub struct GameCamera {
    pos: Vector2,
    pub zoom: f32,
}

impl GameCamera {
    pub fn new() -> Self {
        Self {
            pos: Vector2::zero(),
            zoom: 3.5
        }
    }

    pub fn get_camera(&self, screen_size: Vector2) -> Camera2D {
        Camera2D {
            offset: Vector2::new(screen_size.x / 2.0, screen_size.y / 2.0) - Vector2::new(50.0, 50.0),
            target: self.pos,
            rotation: 0.0,
            zoom: self.zoom,
        }
    }

    pub fn follow(&mut self, target: Vector2, dt: f32) {
        self.pos = lerp(self.pos, target, dt * 2.0);
    }
}

fn lerp(a: Vector2, b: Vector2, t: f32) -> Vector2 {
    a + (b-a) * t
}
