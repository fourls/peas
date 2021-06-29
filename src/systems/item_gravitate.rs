use std::time::Duration;

use specs::prelude::*;

use crate::{
    components::{Item, Player, WorldPosition},
    util::Vec2,
};

const GRAVITATE_SPEED: f32 = 20.;

#[derive(Default)]
pub struct ItemGravitateSystem {}

impl<'s> System<'s> for ItemGravitateSystem {
    type SystemData = (
        ReadExpect<'s, Duration>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Item>,
        WriteStorage<'s, WorldPosition>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (delta, players, items, mut positions) = data;

        let mut player_positions = vec![];

        for (_player, position) in (&players, &positions).join() {
            player_positions.push(position.pos);
        }

        let mut target_pos: Vec2<f32> = Vec2::default();

        for pos in player_positions.iter() {
            target_pos = target_pos + *pos;
        }

        target_pos = target_pos / player_positions.len() as f32;

        for (_item, mut position) in (&items, &mut positions).join() {
            let mut dir = target_pos - position.pos;
            let dist = dir.magnitude();
            dir.normalize();

            let speed =
                GRAVITATE_SPEED * ((dist / 16.).powf(-1.5) - 0.22).clamp(0.0, f32::INFINITY);

            position.pos = position.pos + dir * speed * delta.as_secs_f32();
        }
    }
}
