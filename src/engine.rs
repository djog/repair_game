use raylib::{RaylibHandle, RaylibThread, color::Color, consts::KeyboardKey, prelude::{RaylibDraw, RaylibDrawHandle}, texture::{Image, Texture2D}};

use crate::input::Input;

pub struct Engine {
    rl: RaylibHandle,
    thread: RaylibThread,
}

impl Engine {
    pub fn new(rl: RaylibHandle, thread: RaylibThread) -> Self {
        Self {
            rl,
            thread,
        }
    }

    pub fn window_open(&self) -> bool {
        self.rl.window_should_close()
    }

    pub fn get_input(&mut self) -> Input {
        let input_h = {
            if self.rl.is_key_down(KeyboardKey::KEY_A) && self.rl.is_key_down(KeyboardKey::KEY_D) {
                0.0
            } else if self.rl.is_key_down(KeyboardKey::KEY_D) {
                1.0
            } else if self.rl.is_key_down(KeyboardKey::KEY_A) {
                -1.0
            } else {
                0.0
            }
        };
        let input_v = {
            if self.rl.is_key_down(KeyboardKey::KEY_W) && self.rl.is_key_down(KeyboardKey::KEY_S) {
                0.0
            } else if self.rl.is_key_down(KeyboardKey::KEY_W) {
                1.0
            } else if self.rl.is_key_down(KeyboardKey::KEY_S) {
                -1.0
            } else {
                0.0
            }
        };
        let input_zoom = {
            if self.rl.is_key_down(KeyboardKey::KEY_EQUAL) {
                1.0
            } else if self.rl.is_key_down(KeyboardKey::KEY_MINUS) {
                -1.0
            } else {
                0.0
            }
        };
        let sprint_key = self.rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL);
        let interact_key = self.rl.is_key_down(KeyboardKey::KEY_E);
        let space_key = self.rl.is_key_down(KeyboardKey::KEY_SPACE);
        
        Input {
            input_h,
            input_v,
            input_zoom,
            sprint_key,
            interact_key,
            space_key,
        }
    }

    pub fn get_delta_time(&self) -> f32 {
        self.rl.get_frame_time()
    }

    pub fn create_texture(&mut self, image: Image) -> Texture2D {
        let texture = self
            .rl
            .load_texture_from_image(&self.thread, &image)
            .expect("Failed to crate texture from image!");
        texture
    }

    pub fn start_draw(&mut self, bg_color: Color) -> RaylibDrawHandle {
        let mut d = self.rl.begin_drawing(&self.thread);
        d.clear_background(bg_color);
        return d;
    }
}
