use raylib::{
    color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibMode2DExt},
    texture::{Image, Texture2D},
};

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH, camera::GameCamera, engine::Engine, game_object::GameObject, minigames::{Minigame, MinigameType, build_minigame}, player::Player, world::{World, TILE_SIZE, WORLD_SIZE}};

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
                Vector2::new(1500.0, 200.0),
                MinigameType::Test,
            ),
            GameObject::new(
                Vector2::new(800.0, 200.0),
                MinigameType::Cables,
            )],
        }
    }
}

enum GameState {
    Game,
    MiniGame(MinigameType),
}

pub struct Game {
    engine: Engine,
    data: GameData,
}

impl Game {
    pub fn new() -> Self {
        let (rl, thread) = raylib::init()
            .size(WINDOW_WIDTH, WINDOW_HEIGHT)
            .resizable()
            .title("Repair Game")
            .resizable()
            .build();

        Self {
            engine: Engine::new(rl, thread),
            data: GameData::new(),
        }
    }

    fn init(&mut self) {
        let image = Image::load_image("assets/texture_atlas.png").expect("Couldn't load texture!");
        let texture = self.engine.create_texture(image);
        self.data.texture = Some(texture);

        self.data.world.load_level();
    }

    fn update(&mut self) {
        let dt = self.engine.get_delta_time();

        let input = self.engine.get_input();

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
                self.data.cam.zoom += input.input_zoom * dt * self.data.cam.zoom;
                self.data.cam.follow(self.data.player.pos, dt);
            }
            GameState::MiniGame(_) => {
                if let Some(mg) = &mut self.data.minigame {
                    let switch = mg.update(dt, input);
                    if switch {
                        self.data.state = GameState::Game;
                        self.data.minigame = None;
                    }
                }
            }
        };
    }

    fn draw(&mut self) {
        match self.data.state {
            GameState::Game => {
                let mut d = self.engine.start_draw(Color::WHITE);
                Self::draw_game(&mut d, &self.data);
            }
            GameState::MiniGame(_) => {
                if let Some(mg) = &mut self.data.minigame {
                    mg.draw(&mut self.engine);
                }
            }
        }
    }

    fn draw_game(d: &mut impl RaylibDraw, data: &GameData) {
        {
            let mut d2 = d.begin_mode2D(data.cam.get_camera());

            for x in 0..data.world.tiles.len() {
                for y in 0..data.world.tiles.len() {
                    let tile = data.world.tiles[x][y];

                    let rect = {
                        if tile.id == 0 {
                            Rectangle::new(0.0, 0.0, 64.0, TILE_SIZE as f32)
                        } else if tile.id == 1 {
                            Rectangle::new(0.0, TILE_SIZE as f32, TILE_SIZE as f32, TILE_SIZE as f32)
                        } else if tile.id == 2 {
                            Rectangle::new(TILE_SIZE as f32, TILE_SIZE as f32, TILE_SIZE as f32, TILE_SIZE as f32)
                        } else {
                            Rectangle::new(TILE_SIZE as f32, 0.0, TILE_SIZE as f32, TILE_SIZE as f32)
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
                32,
                32,
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
        while !self.engine.window_open() {
            self.update();
            self.draw();
        }
    }
}
