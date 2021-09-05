use raylib::math::Vector2;

use crate::minigame::MinigameType;

#[derive(Copy, Clone)]
pub struct GameObject {
    pub pos: Vector2,
    pub is_playable: bool,
    pub difficulty: u8,
    pub mg_type: MinigameType,
}

impl GameObject {
    pub fn new(pos: Vector2, mg_type: MinigameType) -> Self {
        Self {
            pos,
            is_playable: true,
            difficulty: 1,
            mg_type,
        }
    }
}
