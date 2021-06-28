use crate::components::*;
use crate::util::Rect;
use crate::util::Vec2;
use specs::prelude::*;

pub fn player(world: &mut World, pos: Vec2<i32>) {
    world
        .create_entity()
        .with(Sprite {
            section: Rect::square(32, 80, 16),
            anchor: Vec2::new(8, 3),
            layer: 5,
        })
        .with(WorldPosition { pos })
        .build();
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum TileType {
    Grass,
    Water,
    Soil,
    Cobble,
}

pub fn tile(world: &mut World, tile_type: TileType, pos: Vec2<i32>) {
    use TileType::*;

    world
        .create_entity()
        .with(Sprite {
            section: Rect::rect(
                match tile_type {
                    Grass => 0,
                    Water => 32,
                    Soil => 64,
                    Cobble => 96,
                },
                match tile_type {
                    _ => 96,
                },
                32,
                16,
            ),
            anchor: Vec2::new(16, 8),
            layer: 1,
        })
        .with(WorldPosition { pos })
        .build();
}
