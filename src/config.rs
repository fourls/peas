use crate::util::{Rect, Vec2};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

lazy_static! {
    pub static ref CONFIG: GameConfig = {
        ron::from_str::<GameConfig>(include_str!("../assets/config.ron"))
            .expect("Could not read game config")
    };
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Sprite {
    pub section: Rect<u32>,
    pub anchor: Vec2<u32>,
    /// The sorting layer of the sprite. Must be in the range 1..10
    pub layer: u8,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub enum ItemId {
    Pea,
    Pod,
    Water,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub enum CropId {
    Pea,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameConfig {
    pub crops: HashMap<CropId, CropSpeciesConfig>,
    pub items: HashMap<ItemId, ItemConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CropSpeciesConfig {
    pub sprites: Vec<Sprite>,
    pub num_stages: usize,
    pub time_between_stages: Vec<f32>,
    pub item_drop: ItemId,
    pub num_items: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemConfig {}

// pub fn get_crop(species: CropSpecies) -> GrowingCrop {
//     match species {
//         CropSpecies::Pea => GrowingCrop {
//             sprites: vec![
//                 Sprite {
//                     section: Rect::rect(64, 64, 32, 16),
//                     anchor: Vec2::new(16, 8),
//                     layer: 5,
//                 },
//                 Sprite {
//                     section: Rect::rect(96, 64, 32, 16),
//                     anchor: Vec2::new(16, 8),
//                     layer: 5,
//                 },
//                 Sprite {
//                     section: Rect::rect(64, 32, 32, 32),
//                     anchor: Vec2::new(16, 8),
//                     layer: 5,
//                 },
//                 Sprite {
//                     section: Rect::rect(96, 32, 32, 32),
//                     anchor: Vec2::new(16, 8),
//                     layer: 5,
//                 },
//             ],
//             num_stages: 4,
//             stage: 0,
//             time_until_next_stage: 1.0,
//             time_between_stages: vec![1.0, 1.0, 1.0, 1.0],
//             item_drop: ItemType::Pea,
//             num_items: 3,
//             species: CropSpecies::Pea,
//         },
//     }
// }
