mod camera;
mod input;
mod player;
mod world;

use camera::GameCamera;
use input::Input;
use player::Player;
use raylib::prelude::*;
use world::{TILE_SIZE, WORLD_SIZE, World};

pub const WINDOW_WIDTH: i32 = 1020;
pub const WINDOW_HEIGHT: i32 = 800;

struct GameData {
    player: Player,
    cam: GameCamera,
    world: World,
    texture: Option<Texture2D>,
}

impl GameData {
    fn new() -> Self {
        Self {
            player: Player::new(),
            cam: GameCamera::new(),
            world: World::new(),
            texture: None,
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
        let i = Image::load_image("texture.png").expect("Couldn't load texture!");
        let texture = self.rl
            .load_texture_from_image(&self.thread, &i)
            .expect("Could not load texture from image!");
        self.data.texture = Some(texture);
    }

    fn get_input(&mut self) -> Input {
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
        Input { input_h, input_v }
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
            d2.draw_circle(0, 0, 64.0, Color::BLUE); // Temp

            for x in 0..data.world.tiles.len() {
                for y in 0..data.world.tiles.len() {
                    let tile = data.world.tiles[x][y];
                    if tile.id >= 0 {
                        let rect = {
                            if tile.id == 0 {
                                Rectangle::new(0.0, 0.0, 64.0, 64.0)
                            } else if tile.id == 2 {
                                Rectangle::new(64.0, 0.0, 64.0, 64.0)
                            } else if tile.id == 3 {
                                Rectangle::new(0.0, 64.0, 64.0, 64.0)
                            } else  {
                                Rectangle::new(64.0, 64.0, 64.0, 64.0)
                            }
                        };
                        let draw_x = x as i32 * TILE_SIZE - (WORLD_SIZE / 2) as i32 * TILE_SIZE;
                        let draw_y = y as i32 * TILE_SIZE - (WORLD_SIZE / 2) as i32 * TILE_SIZE;
                        if let Some(texture_atlas) = &data.texture {
                            d2.draw_texture_rec(
                                texture_atlas,
                                rect,
                                Vector2::new(draw_x as f32, draw_y as f32),
                                Color::WHITE,
                            );
                        };
                    }
                }
            }
            d2.draw_rectangle(
                data.player.pos.x as i32,
                data.player.pos.y as i32,
                100,
                100,
                Color::RED,
            );
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
