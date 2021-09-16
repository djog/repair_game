use super::Minigame;
use crate::{engine::Engine, input::Input};
use rand::{prelude::SliceRandom, thread_rng};
use raylib::{color::Color, prelude::RaylibDraw};

#[derive(Default)]
pub struct CablesGame {
    tiles: Vec<u8>
}

impl CablesGame {
    pub fn new() -> Self {
        let mut tiles = Vec::<u8>::new();
        for x in 0..8 {
            tiles.push(x);
        }
        let mut rng = thread_rng();
        tiles.shuffle(&mut rng);
        println!("De lijst is: {:?}", tiles);
        
        Self {
            tiles
        }
    }
}

impl Minigame for CablesGame {

    fn update(&mut self, _dt: f32, input: Input) -> bool {
        if input.space_key {
            return true;
        }
        false
    }

    fn draw(&self, engine: &mut Engine) {
        let mut d = engine.start_draw(Color::PINK);

        d.draw_text("Cables Game", 100, 100, 54, Color::BLACK);
        d.draw_text("This is an Cable management minigame. Press space to exit.", 100, 160, 24, Color::BLACK);
    }
}
