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

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width) as usize + x as usize
    }

    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            self.tiles[idx] = TileType::Floor;
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            self.tiles[idx] = TileType::Floor;
        }
    }

    pub fn new_map_rooms_and_corridors() -> Map {
        const MAP_SIZE: usize = (MAP_WIDTH * MAP_HEIGHT) as usize;
        let mut map = Map {
            tiles: vec![TileType::Wall; MAP_SIZE],
            rooms: Vec::new(),
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
            revealed_tiles: vec![false; MAP_SIZE],
            visible_tiles: vec![false; MAP_SIZE],
        };

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
            for other_room in map.rooms.iter() {
                if new_room.intersects(other_room) {
                    ok = false;
                }
            }
            if ok {
                map.apply_room_to_map(&new_room);
                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();
                    if rng.range(0, 2) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }

                map.rooms.push(new_room);
            }
        }

        map
    }
}

pub fn draw_map(map: &Map, ctx: &mut Rltk) {
    let mut x = 0;
    let mut y = 0;

    let floor_fg = RGB::from_f32(0.5, 0.5, 0.5);
    let floor_bg = RGB::from_f32(0., 0., 0.);
    let floor_glyph = rltk::to_cp437('.');
    let wall_fg = RGB::from_f32(0., 1.0, 0.);
    let wall_bg = RGB::from_f32(0.5, 0.5, 0.5);
    let wall_glyph = rltk::to_cp437('#');

    for tile in map.tiles.iter() {
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
