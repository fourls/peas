use std::time::Duration;

use specs::prelude::*;

use crate::components::{GrowingCrop, Sprite};

#[derive(Default)]
pub struct CropGrowthSystem;

impl<'s> System<'s> for CropGrowthSystem {
    type SystemData = (
        ReadExpect<'s, Duration>,
        WriteStorage<'s, GrowingCrop>,
        WriteStorage<'s, Sprite>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (delta, mut crops, mut sprites) = data;

        for (crop, sprite) in (&mut crops, &mut sprites).join() {
            if crop.stage + 1 < crop.num_stages {
                crop.time_until_next_stage -= delta.as_secs_f32();

                if crop.time_until_next_stage <= 0.0 {
                    advance_crop(crop, sprite);
                }
            }
        }
    }
}

fn advance_crop(crop: &mut GrowingCrop, sprite: &mut Sprite) {
    crop.stage += 1;

    *sprite = crop.sprites[crop.stage];
    crop.time_until_next_stage = crop.time_between_stages[crop.stage];
}
