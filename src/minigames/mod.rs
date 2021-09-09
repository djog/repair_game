use crate::{engine::Engine, input::Input};
pub use self::empty::EmptyGame;
pub use self::lockpick::LockpickGame;
pub use self::cables::CablesGame;

mod empty;
mod lockpick;
mod cables;


#[derive(Debug, Copy, Clone)]
pub enum MinigameType {
    Test,
    Lockpick,
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
        MinigameType::Lockpick => Box::new(LockpickGame::default()),
        MinigameType::Cables => Box::new(CablesGame::default()),
        MinigameType::Pong => todo!(),
        MinigameType::Test => Box::new(LockpickGame::default()),
        MinigameType::PeanutButterCogs => todo!(),
    }
}