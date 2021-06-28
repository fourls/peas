use crate::components::*;
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
    let pos = coords * crate::TILE_SIZE as i32;

    Vec2::new(pos.x as f32, pos.y as f32)
}

pub fn tile(world: &mut World, tile_type: TileType, coords: Vec2<i32>, blocking: bool) {
    use TileType::*;

    const TILE_COLLIDER_BUFFER: f32 = 2.0;

    let pos_f32 = coords_to_pos(coords);

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
        .with(WorldPosition { pos: pos_f32 });

    if blocking {
        let coll_size = crate::TILE_SIZE as f32;
        builder = builder.with(WorldCollider {
            rect: Rect::square(
                pos_f32.x - coll_size / 2.,
                pos_f32.y - coll_size / 2.,
                coll_size,
            )
            .expand(TILE_COLLIDER_BUFFER),
        });
    }

    builder.build();
}

pub fn crop(world: &mut World, coords: Vec2<i32>) {
    // tile(world, TileType::Soil, coords, false);

    let pos = coords_to_pos(coords);

    const PEA_SHRINK: f32 = 2.0;

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
        time_until_next_stage: 3.0,
        time_between_stages: vec![3.0, 5.0, 4.0, 5.0],
    };

    let coll_size = crate::TILE_SIZE as f32;

    world
        .create_entity()
        .with(pea_crop.sprites[0].clone())
        .with(WorldPosition { pos })
        .with(pea_crop)
        .with(WorldCollider {
            rect: Rect::square(pos.x - coll_size / 2., pos.y - coll_size / 2., coll_size)
                .shrink(PEA_SHRINK),
        })
        .build();
}
