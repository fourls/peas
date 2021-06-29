use specs::prelude::*;
use specs_derive::Component;

use crate::util::{Rect, Vec2};

#[derive(Component, Default)]
pub struct ScreenPosition {
    pub pos: Vec2<i32>,
}

#[derive(Component)]
pub struct WorldPosition {
    pub pos: Vec2<f32>,
}

#[derive(Component, Clone, Copy)]
pub struct Sprite {
    pub section: Rect<u32>,
    pub anchor: Vec2<u32>,
    /// The sorting layer of the sprite. Must be in the range 1..10
    pub layer: u8,
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

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ItemType {
    Pea,
    Pod,
    Water,
}

#[derive(Component)]
pub struct Item {
    pub item_type: ItemType,
}

#[derive(Component)]
pub struct InPlayerInventory {}

#[derive(Component)]
pub struct GrowingCrop {
    pub sprites: Vec<Sprite>,
    pub num_stages: usize,
    pub stage: usize,
    pub time_until_next_stage: f32,
    pub time_between_stages: Vec<f32>,
    pub item_drop: ItemType,
    pub num_items: usize,
}

#[derive(Component, Debug, Clone)]
pub struct Velocity {
    pub vel: Vec2<f32>,
    pub collides: bool,
}
