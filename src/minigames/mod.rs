use crate::{engine::Engine, input::Input};
pub use self::empty::EmptyGame;
pub use self::cables::CablesGame;

mod empty;
mod cables;

#[derive(Debug, Copy, Clone)]
pub enum MinigameType {
    Test,
    Cables,
    Pong,
    PeanutButterCogs,
}

pub trait Minigame {
    fn update(&mut self, dt: f32, input: Input) -> bool;
    fn draw(&self, rl: &mut Engine);
}

// Is this some kind of manual factory pattern?! Idk.. 
pub fn build_minigame(mg_type: MinigameType) -> Box<dyn Minigame> {
    match mg_type {
        MinigameType::Test => Box::new(EmptyGame::default()),
        MinigameType::Cables => Box::new(CablesGame::default()),
        MinigameType::Pong => todo!(),
        MinigameType::PeanutButterCogs => todo!(),
    }
}