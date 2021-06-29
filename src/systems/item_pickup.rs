use specs::prelude::*;

use crate::components::{InPlayerInventory, Item, Player, WorldPosition};

const ITEM_PICKUP_DISTANCE: f32 = 5.0;

#[derive(Default)]
pub struct ItemPickupSystem {}

impl<'s> System<'s> for ItemPickupSystem {
    type SystemData = (
        ReadExpect<'s, LazyUpdate>,
        Entities<'s>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, WorldPosition>,
        ReadStorage<'s, Item>,
        ReadStorage<'s, InPlayerInventory>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (lazy, entities, players, positions, items, in_inventories) = data;

        for (_player, player_pos) in (&players, &positions).join() {
            for (e, _item, item_pos, _) in (&entities, &items, &positions, !&in_inventories).join()
            {
                if player_pos.pos.distance(item_pos.pos) < ITEM_PICKUP_DISTANCE {
                    lazy.exec(move |world| {
                        let mut in_inventories = world.write_storage::<InPlayerInventory>();
                        in_inventories
                            .insert(e, InPlayerInventory {})
                            .expect("Could not add InPlayerInventory");

                        let mut positions = world.write_storage::<WorldPosition>();
                        positions.remove(e);
                    });
                }
            }
        }
    }
}
