use raylib::{
    color::Color,
    consts::KeyboardKey,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibMode2DExt},
    texture::{Image, Texture2D},
    RaylibHandle, RaylibThread,
};

use crate::{
    camera::GameCamera,
    game_object::GameObject,
    input::Input,
    minigame::{build_minigame, Minigame, MinigameType},
    player::Player,
    world::{World, TILE_SIZE, WORLD_SIZE},
    WINDOW_HEIGHT, WINDOW_WIDTH,
};

enum GameState {
    Game,
    MiniGame(MinigameType),
}

struct GameData {
    state: GameState,
    minigame: Option<Box<dyn Minigame>>,
    player: Player,
    cam: GameCamera,
    world: World,
    texture: Option<Texture2D>,
    can_interact: bool,
    interact_msg: String,
    game_objects: Vec<GameObject>,
}

impl GameData {
    fn new() -> Self {
        Self {
            state: GameState::Game,
            minigame: None,
            player: Player::new(),
            cam: GameCamera::new(),
            world: World::new(),
            texture: None,
            can_interact: false,
            interact_msg: String::default(),
            game_objects: vec![GameObject::new(
                Vector2::new(1500.0, 700.0),
                MinigameType::Test,
            )],
        }
    }
}

pub struct Game {
    rl: RaylibHandle,
    thread: RaylibThread,
    data: GameData,
}

impl Game {
    pub fn new() -> Self {
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
        let texture = self
            .rl
            .load_texture_from_image(&self.thread, &i)
            .expect("Could not load texture from image!");
        self.data.texture = Some(texture);

        self.data.world.load_level();
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
        let sprint_key = self.rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL);
        let interact_key = self.rl.is_key_down(KeyboardKey::KEY_E);
        Input {
            input_h,
            input_v,
            sprint_key,
            interact_key,
        }
    }

    fn update(&mut self) {
        let dt = self.rl.get_frame_time();

        let input = self.get_input();

        match self.data.state {
            GameState::Game => {
                self.data.player.update(dt, input);
                self.data.can_interact = false;
                for go in self.data.game_objects.iter() {
                    if self.data.player.pos.distance_to(go.pos) < 200.0 {
                        if go.is_playable {
                            self.data.can_interact = true;
                            self.data.interact_msg = format!("Go to {:?}", go.mg_type);
                            if input.interact_key {
                                // Set minigame
                                self.data.state = GameState::MiniGame(go.mg_type.clone());
                                self.data.minigame = Some(build_minigame(go.mg_type));
                            }
                        }
                    }
                }
                // Camera stuff
                let zoom_input = {
                    if self.rl.is_key_down(KeyboardKey::KEY_EQUAL) {
                        1.0
                    } else if self.rl.is_key_down(KeyboardKey::KEY_MINUS) {
                        -1.0
                    } else {
                        0.0
                    }
                };
                self.data.cam.zoom += zoom_input * dt * self.data.cam.zoom;
                self.data.cam.follow(self.data.player.pos, dt);
            }
            GameState::MiniGame(mg_type) => {
                if let Some(mg) = &mut self.data.minigame {
                    mg.update(dt, input);
                }
                println!("UPDATE: {:?}", mg_type);
            }
        };
    }

    fn draw(&mut self) {
        let mut d = self.rl.begin_drawing(&self.thread);

        d.clear_background(Color::WHITE);

        Self::draw_game(&mut d, &self.data);
    }

    fn draw_game(d: &mut impl RaylibDraw, data: &GameData) {
        {
            let mut d2 = d.begin_mode2D(data.cam.get_camera());

            for x in 0..data.world.tiles.len() {
                for y in 0..data.world.tiles.len() {
                    let tile = data.world.tiles[x][y];

                    let rect = {
                        if tile.id == 0 {
                            Rectangle::new(0.0, 0.0, 64.0, 64.0)
                        } else if tile.id == 1 {
                            Rectangle::new(0.0, 64.0, 64.0, 64.0)
                        } else if tile.id == 2 {
                            Rectangle::new(64.0, 64.0, 64.0, 64.0)
                        } else {
                            Rectangle::new(64.0, 0.0, 64.0, 64.0)
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

            for part in data.game_objects.iter() {
                d2.draw_circle(
                    part.pos.x as i32,
                    part.pos.y as i32,
                    TILE_SIZE as f32,
                    Color::YELLOW,
                );
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
        if data.can_interact {
            d.draw_text(
                &format!("[E] {}", data.interact_msg),
                WINDOW_WIDTH / 2,
                WINDOW_HEIGHT - 50,
                24,
                Color::ORANGE,
            );
        }
    }

    pub fn run(mut self) {
        self.init();
        while !self.rl.window_should_close() {
            self.update();
            self.draw();
        }
    }
}
