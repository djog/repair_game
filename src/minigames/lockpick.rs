use super::Minigame;
use crate::{engine::Engine, input::Input};
use raylib::{color::Color, prelude::RaylibDraw};

#[derive(Default)]
pub struct LockpickGame;

impl Minigame for LockpickGame { 
    fn update(&mut self, _dt: f32, input: Input) -> bool {
        if input.space_key {
            return true;
        }
        false
    }

    fn draw(&self, engine: &mut Engine) {
        let mut d = engine.start_draw(Color::YELLOW);

        d.draw_text("Empty Minigame", 100, 100, 54, Color::BLACK);
        d.draw_text("This is an empty minigame. Press space to exit.", 100, 160, 24, Color::BLACK);
    }
}
