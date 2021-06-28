use std::time::Duration;

use specs::prelude::*;

use crate::{
    components::{Player, WorldCollider, WorldPosition},
    input::{Input, Key},
    util::Vec2,
};

#[derive(Default)]
pub struct PlayerMovementSystem;

impl<'s> System<'s> for PlayerMovementSystem {
    type SystemData = (
        ReadExpect<'s, Input>,
        ReadExpect<'s, Duration>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, WorldPosition>,
        ReadStorage<'s, WorldCollider>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (input, delta, players, mut positions, colliders) = data;

        for (_player, position) in (&players, &mut positions).join() {
            let mut axis: Vec2<f32> = Vec2::default();

            if input.is_down(Key::W) {
                axis.y += 1.0;
            }
            if input.is_down(Key::S) {
                axis.y -= 1.0;
            }
            if input.is_down(Key::A) {
                axis.x -= 1.0;
            }
            if input.is_down(Key::D) {
                axis.x += 1.0;
            }

            axis.normalize();

            let new_pos: Vec2<f32> =
                position.pos + axis * crate::PLAYER_SPEED * delta.as_secs_f32();

            let mut safe = true;

            for collider in (&colliders).join() {
                if collider.rect.inside(new_pos) {
                    safe = false;
                    break;
                }
            }

            if safe {
                position.pos = new_pos;
            }
        }
    }
}
