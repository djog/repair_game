use crate::input::Input;

#[derive(Debug, Copy, Clone)]
pub enum MinigameType {
    Test,
    Pong,
    PeanutButterCogs,
}

pub trait Minigame {
    fn update(&mut self, dt: f32, input: Input);
    // fn draw(&self, d: &mut impl RaylibDraw);
}

// Is this some kind of manual factory pattern?! Idk.. 
pub fn build_minigame(mg_type: MinigameType) -> Box<dyn Minigame> {
    match mg_type {
        MinigameType::Test => Box::new(TestGame::default()),
        MinigameType::Pong => todo!(),
        MinigameType::PeanutButterCogs => todo!(),
    }
}

#[derive(Default)]
pub struct TestGame {
    count: i32
}

impl Minigame for TestGame {
    fn update(&mut self, _dt: f32, _input: Input) {
        self.count += 1;
        println!("Update TEST count is: {}", self.count);
    }
}