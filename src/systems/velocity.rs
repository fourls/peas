use std::time::Duration;

use specs::prelude::*;

use crate::components::{PreventsMovement, Velocity, WorldCollider, WorldPosition};

#[derive(Default)]
pub struct VelocitySystem {}

impl<'s> System<'s> for VelocitySystem {
    type SystemData = (
        ReadExpect<'s, Duration>,
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, WorldPosition>,
        ReadStorage<'s, WorldCollider>,
        ReadStorage<'s, PreventsMovement>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (delta, velocitys, mut positions, colliders, walls) = data;

        for (velocity, position) in (&velocitys, &mut positions).join() {
            let new_pos = position.pos + velocity.vel * delta.as_secs_f32();

            if velocity.collides {
                let mut ok = true;
                for (collider, _wall) in (&colliders, &walls).join() {
                    if collider.rect.contains(new_pos) {
                        ok = false;
                        break;
                    }
                }

                if !ok {
                    continue;
                }
            }

            position.pos = new_pos;
        }
    }
}
