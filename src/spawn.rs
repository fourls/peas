use crate::components::*;
use crate::util::Rect;
use specs::prelude::*;

pub fn player(world: &mut World, pos: (i32, i32)) {
    world
        .create_entity()
        .with(Sprite {
            section: Rect::square(32, 80, 16),
            anchor: (8, 1),
            layer: 5,
        })
        .with(Position::default())
        .with(TilePosition { x: pos.0, y: pos.1 })
        .build();
}

pub enum TileType {
    Grass,
    Water,
    Soil,
    Cobble,
}

pub fn tile(world: &mut World, tile_type: TileType, pos: (i32, i32)) {
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
            anchor: (16, 8),
            layer: 1,
        })
        .with(Position::default())
        .with(TilePosition { x: pos.0, y: pos.1 })
        .build();
}
