use specs::prelude::*;

use crate::{
    components::{Player, WorldCollider, WorldPosition},
    input::{Input, Key},
    util::Vec2,
    TILE_SIZE,
};

#[derive(Default)]
pub struct PlayerMovementSystem;

impl<'s> System<'s> for PlayerMovementSystem {
    type SystemData = (
        ReadExpect<'s, Input>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, WorldPosition>,
        ReadStorage<'s, WorldCollider>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (input, players, mut positions, colliders) = data;

        for (_player, position) in (&players, &mut positions).join() {
            let mut axis = Vec2::default();

            if input.is_just_pressed(Key::W) {
                axis.y += 1;
            }
            if input.is_just_pressed(Key::S) {
                axis.y -= 1;
            }
            if input.is_just_pressed(Key::A) {
                axis.x -= 1;
            }
            if input.is_just_pressed(Key::D) {
                axis.x += 1;
            }

            position.pos += axis * TILE_SIZE as i32;
        }
    }
}
