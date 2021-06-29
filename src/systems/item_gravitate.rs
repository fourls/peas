use std::time::Duration;

use specs::prelude::*;

use crate::{
    components::{Item, Player, Velocity, WorldPosition},
    util::Vec2,
};

const GRAVITATE_SPEED: f32 = 20.;
const GRAVITATE_ACCEL: f32 = 2.;

#[derive(Default)]
pub struct ItemGravitateSystem {}

impl<'s> System<'s> for ItemGravitateSystem {
    type SystemData = (
        ReadExpect<'s, Duration>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Item>,
        ReadStorage<'s, WorldPosition>,
        WriteStorage<'s, Velocity>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (delta, players, items, positions, mut velocities) = data;

        let mut player_positions = vec![];

        for (_player, position) in (&players, &positions).join() {
            player_positions.push(position.pos);
        }

        let mut target_pos: Vec2<f32> = Vec2::default();

        for pos in player_positions.iter() {
            target_pos = target_pos + *pos;
        }

        target_pos = target_pos / player_positions.len() as f32;

        for (_item, position, mut velocity) in (&items, &positions, &mut velocities).join() {
            let mut dir = target_pos - position.pos;
            let dist = dir.magnitude();
            dir.normalize();

            let speed =
                GRAVITATE_SPEED * ((dist / 16.).powf(-1.5) - 0.22).clamp(0.0, f32::INFINITY);

            velocity.vel = dir * speed;
        }
    }
}
