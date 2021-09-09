// Modules
mod camera;
mod engine;
mod game;
mod game_object;
mod input;
mod minigames;
mod player;
mod world;

pub const WINDOW_WIDTH: i32 = 1020;
pub const WINDOW_HEIGHT: i32 = 800;

use game::Game;

fn main() {
    Game::new().run();
}
