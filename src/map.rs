use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TiltType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TiltType>,
}

pub fn map_ids(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    pub fn new() -> Self {
        Map {
            tiles: vec![TiltType::Floor; NUM_TILES],
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_ids(x, y);
                match self.tiles[idx] {
                    TiltType::Floor => {
                        ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
                    }
                    TiltType::Wall => {
                        ctx.set(x, y, GREEN, BLACK, to_cp437('#'));
                    }
                }
            }
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_ids(point.x, point.y)] == TiltType::Floor
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_ids(point.x, point.y))
        }
    }
}
