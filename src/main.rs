mod input;
mod player;
mod camera;

use input::Input;
use player::Player;
use raylib::prelude::*;
use camera::GameCamera;

pub const WINDOW_WIDTH: i32 = 1020;
pub const WINDOW_HEIGHT : i32 = 800;

struct GameData {
    player: Player,
    cam: GameCamera,
}

impl GameData {
    fn new() -> Self {
        Self {
            player: Player::new(),
            cam: GameCamera::new()
        }
    }
}

struct Game {
    rl: RaylibHandle,
    thread: RaylibThread,
    data: GameData,
}

impl Game {
    fn new() -> Self {
        let (rl, thread) = raylib::init()
            .size(WINDOW_WIDTH, WINDOW_HEIGHT)
            .title("Repair Game")
            .build();

        Self {
            rl,
            thread,
            data: GameData::new(),
        }
    }
    
    fn init(&mut self) {
         
    }

    fn get_input(&mut self) -> Input 
    {
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
        Input {
            input_h,
            input_v
        }
    }

    fn update(&mut self) {
        let input = self.get_input();
        let dt = self.rl.get_frame_time();
        self.data.player.update(dt, input);
        self.data.cam.follow(self.data.player.pos, dt);
    }

    fn draw(&mut self) {
        let mut d = self.rl.begin_drawing(&self.thread);

        d.clear_background(Color::WHITE);

        Self::draw_game(&mut d, &self.data);
    }

    fn draw_game(d: &mut impl RaylibDraw, data: &GameData) {
        {
            let mut d2 = d.begin_mode2D(data.cam.get_camera());
            d2.draw_rectangle(data.player.pos.x as i32, data.player.pos.y as i32, 100, 100, Color::RED);
            
            d2.draw_circle(500, 500, 64.0, Color::BLUE); // Temp
        }
        d.draw_fps(12, 12);
    }

    fn run(mut self) {
        self.init();
        while !self.rl.window_should_close() {
            self.update();
            self.draw();
        }
    }
}

fn main() {
    Game::new().run();
}
