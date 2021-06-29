use std::f32::consts::PI;

use rand::prelude::ThreadRng;
use rand::Rng;
use specs::prelude::*;

use crate::config::CONFIG;
use crate::{
    components::{GrowingCrop, WorldClickable, WorldPosition},
    resources::{input::MouseButton, Camera, Input},
    spawn,
    util::Vec2,
};

const CROP_ITEM_RADIUS: f32 = 5.0;
const CROP_ITEM_VARIATION: f32 = 2.7;

#[derive(Default)]
pub struct CropHarvestSystem;

impl<'s> System<'s> for CropHarvestSystem {
    type SystemData = (
        Entities<'s>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, Input>,
        ReadExpect<'s, Camera>,
        ReadStorage<'s, WorldPosition>,
        ReadStorage<'s, GrowingCrop>,
        ReadStorage<'s, WorldClickable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, update, input, camera, positions, crops, clickables) = data;

        if !input.is_mouse_just_pressed(MouseButton::Left) {
            return;
        }

        let world_mouse = camera.screen_to_world_pp(&camera.view_to_screen_pp(&input.mouse_pos()));
        let world_mouse = Vec2::new(world_mouse.x as f32, world_mouse.y as f32);

        for (e, crop, position, clickable) in (&entities, &crops, &positions, &clickables).join() {
            // ignore all crops that are not fully grown
            if crop.stage + 1 != CONFIG.crops[&crop.species].num_stages {
                return;
            }

            if clickable.rect.contains(world_mouse) {
                let pos_vec = position.pos;
                let crop_num_items = CONFIG.crops[&crop.species].num_items;
                let crop_item_drop = CONFIG.crops[&crop.species].item_drop;

                update.exec(move |world| {
                    world.delete_entity(e).expect("Could not delete entity");

                    let mut random = rand::thread_rng();

                    let circle_pos = |angle: f32| {
                        pos_vec
                            + Vec2::new(
                                angle.cos() * CROP_ITEM_RADIUS,
                                angle.sin() * CROP_ITEM_RADIUS,
                            )
                    };

                    let mut angle;
                    let circle_division = (2.0 * PI) / crop_num_items as f32;

                    for n in 0..crop_num_items {
                        angle = circle_division * n as f32;
                        angle = random_range(
                            &mut random,
                            angle - circle_division / CROP_ITEM_VARIATION,
                            angle + circle_division / CROP_ITEM_VARIATION,
                        );
                        spawn::item(world, crop_item_drop, circle_pos(angle));
                    }
                });
            }
        }
    }
}

fn random_range(random: &mut ThreadRng, min: f32, max: f32) -> f32 {
    min + (max - min) * random.gen::<f32>()
}
