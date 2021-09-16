// Modules
mod camera;
mod engine;
mod game;
mod game_object;
mod input;
mod minigames;
mod physics;
mod player;
mod world;

use game::Game;

fn main() {
    Game::new().run();
}
