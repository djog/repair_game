use raylib::prelude::*;

const WINDOW_WIDTH: i32 = 1020;
const WINDOW_HEIGHT: i32 = 800;

struct Game {
    rl: RaylibHandle,
    thread: RaylibThread,
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
        }
    }
    
    fn init(&mut self) {
         
    }

    fn update(&mut self) {

    }

    fn draw(&mut self) {
        let mut d = self.rl.begin_drawing(&self.thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Repair game!", 12, 12, 46, Color::BLACK);                
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
