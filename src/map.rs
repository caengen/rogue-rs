use crate::rect::*;
use rltk::{Console, RandomNumberGenerator, Rltk, Tile, RGB};
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};

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

pub fn new_map_test() -> Vec<TileType> {
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

fn apply_room_to_map(room: &Rect, map: &mut [TileType]) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            map[xy_idx(x, y)] = TileType::Floor;
        }
    }
}

fn apply_horizontal_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = xy_idx(x, y);
        map[idx] = TileType::Floor;
    }
}

fn apply_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = xy_idx(x, y);
        map[idx] = TileType::Floor;
    }
}

pub fn new_map_rooms_and_corridors() -> Vec<TileType> {
    let mut map = vec![TileType::Wall; (MAP_WIDTH * MAP_HEIGHT) as usize];

    let mut rooms: Vec<Rect> = Vec::new();
    const MAX_ROOMS: i32 = 30;
    const MAX_SIZE: i32 = 10;
    const MIN_SIZE: i32 = 6;

    let mut rng = RandomNumberGenerator::new();

    for _ in 0..MAX_ROOMS {
        let w = rng.range(MIN_SIZE, MAX_SIZE);
        let h = rng.range(MIN_SIZE, MAX_SIZE);
        let x = rng.roll_dice(1, MAP_WIDTH - w - 1) - 1;
        let y = rng.roll_dice(1, MAP_HEIGHT - h - 1) - 1;
        let new_room = Rect::new(x, y, w, h);
        let mut ok = true;
        for other_room in rooms.iter() {
            if new_room.intersects(other_room) {
                ok = false;
            }
        }
        if ok {
            apply_room_to_map(&new_room, &mut map);
            rooms.push(new_room);
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
