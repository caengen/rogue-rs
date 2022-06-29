use rltk::{Console, RandomNumberGenerator, Rltk, Tile, RGB};
use specs::prelude::*;
use specs_derive::Component;

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 50;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y * MAP_WIDTH) as usize + x as usize
}

pub fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; (MAP_WIDTH * MAP_HEIGHT) as usize];

    for x in 0..MAP_WIDTH {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, MAP_HEIGHT - 1)] = TileType::Wall;
    }

    for y in 0..MAP_HEIGHT {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(MAP_WIDTH - 1, y)] = TileType::Wall;
    }

    let mut rng = rltk::RandomNumberGenerator::new();

    let player_start_pos = xy_idx(40, 25);
    for _i in 0..400 {
        let x = rng.roll_dice(1, MAP_WIDTH - 1);
        let y = rng.roll_dice(1, MAP_HEIGHT - 1);
        let idx = xy_idx(x, y);

        if idx != player_start_pos {
            map[idx] = TileType::Wall;
        }
    }

    map
}

pub fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    let mut x = 0;
    let mut y = 0;

    let floor_fg = RGB::from_f32(0.5, 0.5, 0.5);
    let floor_bg = RGB::from_f32(0., 0., 0.);
    let floor_glyph = rltk::to_cp437('.');
    let wall_fg = RGB::from_f32(0., 1.0, 0.);
    let wall_bg = RGB::from_f32(0.5, 0.5, 0.5);
    let wall_glyph = rltk::to_cp437('#');

    for tile in map.iter() {
        match tile {
            TileType::Floor => {
                ctx.set(x, y, floor_fg, floor_bg, floor_glyph);
            }
            TileType::Wall => {
                ctx.set(x, y, wall_fg, wall_bg, wall_glyph);
            }
        }

        x += 1;
        if x == MAP_WIDTH {
            x = 0;
            y += 1;
        }
    }
}
