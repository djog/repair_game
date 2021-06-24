#[derive(Default, Clone, Copy)]
pub struct Tile {
    pub id: u8,
}

pub const TILE_SIZE: i32 = 32;
pub const WORLD_SIZE: usize = 128; 

pub struct World {
    pub tiles: [[Tile; 128]; 128],
}

impl World {
    pub fn new() -> Self {
        Self {
            tiles: [[Tile::default(); 128]; 128]
        }
    }
}