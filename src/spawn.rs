use crate::components::*;
use crate::constants::TILE_SIZE;
use crate::util::*;
use specs::prelude::*;

pub fn player(world: &mut World, pos: Vec2<f32>) {
    world
        .create_entity()
        .with(Player {})
        .with(Sprite {
            section: Rect::square(32, 80, 16),
            anchor: Vec2::new(8, 1),
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

fn coords_to_pos(coords: Vec2<i32>) -> Vec2<f32> {
    let pos = coords * TILE_SIZE as i32;

    Vec2::new(pos.x as f32, pos.y as f32)
}

pub fn tile(world: &mut World, tile_type: TileType, coords: Vec2<i32>, blocking: bool) {
    use TileType::*;

    const TILE_COLLIDER_BUFFER: f32 = 2.0;

    let pos_f32 = coords_to_pos(coords);
    let coll_size = TILE_SIZE as f32;

    let mut builder = world
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
        .with(WorldPosition { pos: pos_f32 })
        .with(WorldCollider {
            rect: Rect::square(
                pos_f32.x - coll_size / 2.,
                pos_f32.y - coll_size / 2.,
                coll_size,
            )
            .expand(TILE_COLLIDER_BUFFER),
        });

    if blocking {
        builder = builder.with(PreventsMovement {});
    }

    builder.build();
}

pub fn crop(world: &mut World, coords: Vec2<i32>) {
    // tile(world, TileType::Soil, coords, false);

    let pos = coords_to_pos(coords);

    let pea_crop = GrowingCrop {
        sprites: vec![
            Sprite {
                section: Rect::rect(64, 64, 32, 16),
                anchor: Vec2::new(16, 8),
                layer: 5,
            },
            Sprite {
                section: Rect::rect(96, 64, 32, 16),
                anchor: Vec2::new(16, 8),
                layer: 5,
            },
            Sprite {
                section: Rect::rect(64, 32, 32, 32),
                anchor: Vec2::new(16, 8),
                layer: 5,
            },
            Sprite {
                section: Rect::rect(96, 32, 32, 32),
                anchor: Vec2::new(16, 8),
                layer: 5,
            },
        ],
        num_stages: 4,
        stage: 0,
        time_until_next_stage: 1.0,
        time_between_stages: vec![1.0, 1.0, 1.0, 1.0],
        item_drop: ItemType::Pea,
        num_items: 3,
    };

    const PEA_SIZE: f32 = TILE_SIZE as f32;
    const PEA_SHRINK: f32 = 2.0;

    let coll_rect =
        Rect::square(pos.x - PEA_SIZE / 2., pos.y - PEA_SIZE / 2., PEA_SIZE).shrink(PEA_SHRINK);

    world
        .create_entity()
        .with(pea_crop.sprites[0].clone())
        .with(WorldPosition { pos })
        .with(pea_crop)
        .with(WorldCollider { rect: coll_rect })
        .with(PreventsMovement {})
        .with(WorldClickable { rect: coll_rect })
        .build();
}

pub fn item(world: &mut World, item_type: ItemType, pos: Vec2<f32>) {
    let coll_size = match item_type {
        _ => 4.0,
    };

    let coll_rect = Rect::square(pos.x - coll_size / 2., pos.y - coll_size / 2., coll_size);

    world
        .create_entity()
        .with(WorldPosition { pos })
        .with(WorldCollider { rect: coll_rect })
        .with(Sprite {
            section: Rect::square(48, 80, 16),
            anchor: Vec2::new(8, 0),
            layer: 5,
        })
        .with(Item { item_type })
        .build();
}
