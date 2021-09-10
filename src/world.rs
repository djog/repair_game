use std::fs;

#[derive(Default, Clone, Copy)]
pub struct Tile {
    pub id: u8,
}

pub const TILE_SIZE: i32 = 16;
pub const WORLD_SIZE: usize = 128;
pub const LEVEL_FILE: &str = "level.txt";

pub struct World {
    pub tiles: [[Tile; WORLD_SIZE]; WORLD_SIZE],
}

impl World {
    pub fn new() -> Self {
        Self {
            tiles: [[Tile::default(); WORLD_SIZE]; WORLD_SIZE],
        }
    }

    pub fn _generate(&mut self) {
        for x in 0..WORLD_SIZE {
            for y in 0..WORLD_SIZE {
                if x % 2 == 0 {
                    self.tiles[x][y] = Tile { id: 1 };
                } else {
                    self.tiles[x][y] = Tile { id: 0 };
                }
            }
        }
    }

    pub fn load_level(&mut self) {
        let contents =
            fs::read_to_string(LEVEL_FILE).expect("Something went wrong reading the level file!");
        let mut line_counter = 0;
        for l in contents.lines() {
            for c in 0..l.len() {
                let id = l.chars().nth(c).unwrap() as u8 - 48;
                self.tiles[c][line_counter] = Tile { id };
            }
            line_counter += 1;
        }
    }
}
