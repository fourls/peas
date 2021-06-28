use crate::components::*;
use crate::util::*;
use specs::prelude::*;

pub fn player(world: &mut World, pos: Vec2<i32>) {
    world
        .create_entity()
        .with(Player {})
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

pub fn tile(world: &mut World, tile_type: TileType, coords: Vec2<i32>) {
    use TileType::*;

    let pos = coords * crate::TILE_SIZE as i32;

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
        .with(WorldCollider {
            rect: Rect::square(pos.x, pos.y, crate::TILE_SIZE as i32),
        })
        .build();
}
