use raylib::prelude::*;

const WINDOW_WIDTH: i32 = 1020;
const WINDOW_HEIGHT : i32 = 800;

#[derive(Debug)]
struct Input {
    input_v: f32,
    input_h: f32,
}

struct Player {
    pos: Vector2,
} 

impl Player {
    fn new() -> Self {
        Self {
            pos: Vector2::zero()
        }
    }

    fn update(&mut self, delta_time: f32, input: Input) {
        self.pos += Vector2::new(input.input_h, input.input_v) * delta_time * 100.0;
    }
}

struct GameData {
    player: Player,
}

impl GameData {
    fn new() -> Self {
        Self {
            player: Player::new()
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
        let (mut rl, thread) = raylib::init()
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

            if self.rl.is_key_pressed(KeyboardKey::KEY_RIGHT) && self.rl.is_key_pressed(KeyboardKey::KEY_LEFT) {
                0.0
            } else if self.rl.is_key_pressed(KeyboardKey::KEY_RIGHT) {
                1.0
            } else if self.rl.is_key_pressed(KeyboardKey::KEY_LEFT) {
                -1.0
            } else {
                0.0
            }
        };

        let input_v = 0.0;

        Input {
            input_h,
            input_v
        }
    }

    fn update(&mut self) {
        let input = self.get_input();
        println!("{:?}", input); 
        let dt = self.rl.get_frame_time();
        self.data.player.update(dt, input);
    }

    fn draw(&mut self) {
        let mut d = self.rl.begin_drawing(&self.thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Repair game!", 12, 12, 46, Color::BLACK);                
        Self::draw_game(&mut d, &self.data);
    }

    fn draw_game(d: &mut impl RaylibDraw, data: &GameData) {
        d.draw_rectangle(data.player.pos.x as i32, data.player.pos.y as i32, 100, 100, Color::RED);
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
