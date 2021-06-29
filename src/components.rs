use serde::Deserialize;
use specs::prelude::*;
use specs_derive::Component;

use crate::{
    config::{CropId, ItemId, Sprite},
    util::{Rect, Vec2},
};

#[derive(Component, Default)]
pub struct ScreenPosition {
    pub pos: Vec2<i32>,
}

#[derive(Component)]
pub struct WorldPosition {
    pub pos: Vec2<f32>,
}

#[derive(Component, Clone)]
pub struct Renderable {
    pub sprite: Sprite,
}

#[derive(Component)]
pub struct PreventsMovement {}

#[derive(Component)]
pub struct WorldCollider {
    pub rect: Rect<f32>,
}

#[derive(Component)]
pub struct WorldClickable {
    pub rect: Rect<f32>,
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Item {
    pub item_type: ItemId,
}

#[derive(Component)]
pub struct InPlayerInventory {}

#[derive(Component)]
pub struct GrowingCrop {
    pub sprites: Vec<Sprite>,
    pub stage: usize,
    pub time_until_next_stage: f32,
    pub species: CropId,
}

#[derive(Component, Debug, Clone)]
pub struct Velocity {
    pub vel: Vec2<f32>,
    pub collides: bool,
}
