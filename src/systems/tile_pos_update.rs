use std::f32::consts::PI;

use cgmath::{Matrix2, Matrix3, Vector2, Vector3};
use specs::prelude::*;

use crate::components::{Position, TilePosition};

pub const TILE_SIZE: i32 = 16;
pub const SCREEN_WIDTH: i32 = 400 / 4;
pub const SCREEN_HEIGHT: i32 = 400 / 4;

pub struct TilePositionUpdateSystem {}

impl<'s> System<'s> for TilePositionUpdateSystem {
    type SystemData = (ReadStorage<'s, TilePosition>, WriteStorage<'s, Position>);

    fn run(&mut self, data: Self::SystemData) {
        let (tile_pos, mut real_pos) = data;

        for (tile_pos, real_pos) in (&tile_pos, &mut real_pos).join() {
            let vec = world_to_view(&Vector2::new(
                tile_pos.x * TILE_SIZE,
                tile_pos.y * TILE_SIZE,
            ));

            *real_pos = Position { x: vec.x, y: vec.y };
        }
    }
}

pub fn world_to_view(world_pos: &Vector2<i32>) -> Vector2<i32> {
    Vector2::new(
        world_pos.x - world_pos.y + SCREEN_WIDTH / 2,
        (world_pos.x + world_pos.y) / 2 + SCREEN_HEIGHT / 2,
    )
}
