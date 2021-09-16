use raylib::math::Vector2;

use crate::world::*;

pub struct AABB {
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}
  
fn screen_to_grid_coords(screen_coords: &Vector2) -> Vector2 {
    return Vector2::new(screen_coords.x / TILE_SIZE as f32, screen_coords.y / TILE_SIZE as f32);
}

const CHECK_RADIUS: i32 = 4;
  
fn physics(player_pos: &Vector2, world: &World) {
    let player_grid_pos = screen_to_grid_coords(player_pos);
    let center_x = player_grid_pos.x as i32;
    let center_y = player_grid_pos.y as i32;

    let colliders = Vec::<AABB>::new();
    for x_offset in -CHECK_RADIUS..CHECK_RADIUS {
        for y_offset in -CHECK_RADIUS..CHECK_RADIUS {
            let x = (center_x + x_offset).clamp(0, WORLD_SIZE as i32);
            let y = (center_y + y_offset).clamp(0, WORLD_SIZE as i32);
            let tile = world.tiles[x as usize][y as usize];
            if tile.id > 0 {
                colliders.push();
            }
        }
    }
}