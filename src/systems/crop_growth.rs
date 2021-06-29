use std::time::Duration;

use specs::prelude::*;

use crate::components::{GrowingCrop, Renderable};
use crate::config::{GameConfig, CONFIG};

#[derive(Default)]
pub struct CropGrowthSystem;

impl<'s> System<'s> for CropGrowthSystem {
    type SystemData = (
        ReadExpect<'s, Duration>,
        WriteStorage<'s, GrowingCrop>,
        WriteStorage<'s, Renderable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (delta, mut crops, mut sprites) = data;

        for (crop, renderable) in (&mut crops, &mut sprites).join() {
            if crop.stage + 1 < CONFIG.crops[&crop.species].num_stages {
                crop.time_until_next_stage -= delta.as_secs_f32();

                if crop.time_until_next_stage <= 0.0 {
                    advance_crop(crop, renderable, &CONFIG);
                }
            }
        }
    }
}

fn advance_crop(crop: &mut GrowingCrop, renderable: &mut Renderable, config: &GameConfig) {
    crop.stage += 1;

    renderable.sprite = crop.sprites[crop.stage].clone();
    crop.time_until_next_stage = config.crops[&crop.species].time_between_stages[crop.stage];
}
