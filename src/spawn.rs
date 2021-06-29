use crate::components::*;
use crate::config::CropId;
use crate::config::ItemId;
use crate::config::Sprite;
use crate::config::CONFIG;
use crate::constants::TILE_SIZE;
use crate::util::*;
use specs::prelude::*;

pub fn player(world: &mut World, pos: Vec2<f32>) {
    world
        .create_entity()
        .with(Player {})
        .with(Renderable {
            sprite: Sprite {
                section: Rect::square(32, 0, 16),
                anchor: Vec2::new(8, 1),
                layer: 5,
            },
        })
        .with(WorldPosition { pos })
        .with(Velocity {
            collides: true,
            vel: Vec2::default(),
        })
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
        .with(Renderable {
            sprite: Sprite {
                section: Rect::rect(
                    0,
                    match tile_type {
                        Grass => 0,
                        Water => 16,
                        Soil => 32,
                        Cobble => 48,
                    },
                    32,
                    16,
                ),
                anchor: Vec2::new(16, 8),
                layer: 1,
            },
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

pub fn crop(world: &mut World, species: CropId, coords: Vec2<i32>) {
    // tile(world, TileType::Soil, coords, false);

    let pos = coords_to_pos(coords);

    let crop_info = &CONFIG.crops[&species];

    const CROP_SIZE: f32 = TILE_SIZE as f32;
    const CROP_SHRINK: f32 = 2.0;

    let coll_rect =
        Rect::square(pos.x - CROP_SIZE / 2., pos.y - CROP_SIZE / 2., CROP_SIZE).shrink(CROP_SHRINK);

    world
        .create_entity()
        .with(GrowingCrop {
            species,
            sprites: crop_info.sprites.clone(),
            stage: 0,
            time_until_next_stage: crop_info.time_between_stages[0],
        })
        .with(WorldPosition { pos })
        .with(Renderable {
            sprite: crop_info.sprites[0].clone(),
        })
        .with(WorldCollider { rect: coll_rect })
        .with(PreventsMovement {})
        .with(WorldClickable { rect: coll_rect })
        .build();
}

pub fn item(world: &mut World, item_type: ItemId, pos: Vec2<f32>) {
    let coll_size = match item_type {
        _ => 4.0,
    };

    let coll_rect = Rect::square(pos.x - coll_size / 2., pos.y - coll_size / 2., coll_size);

    world
        .create_entity()
        .with(WorldPosition { pos })
        .with(WorldCollider { rect: coll_rect })
        .with(Renderable {
            sprite: Sprite {
                section: Rect::square(32, 48, 16),
                anchor: Vec2::new(8, 0),
                layer: 5,
            },
        })
        .with(Item { item_type })
        .with(Velocity {
            collides: false,
            vel: Vec2::default(),
        })
        .build();
}
