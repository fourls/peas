use std::time::Duration;

use specs::prelude::*;

use crate::{
    components::{Player, PreventsMovement, Velocity, WorldCollider, WorldPosition},
    constants::PLAYER_SPEED,
    resources::{input::Key, Input},
    util::Vec2,
};

#[derive(Default)]
pub struct PlayerMovementSystem;

impl<'s> System<'s> for PlayerMovementSystem {
    type SystemData = (
        ReadExpect<'s, Input>,
        ReadExpect<'s, Duration>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, WorldCollider>,
        ReadStorage<'s, PreventsMovement>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (input, delta, players, mut velocities, colliders, walls) = data;

        for (_player, velocity) in (&players, &mut velocities).join() {
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

            velocity.vel = axis * PLAYER_SPEED;
        }
    }
}
