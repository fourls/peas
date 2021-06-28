use specs::prelude::*;
use specs_derive::Component;

use crate::util::{Rect, Vec2};

#[derive(Component, Default)]
pub struct ScreenPosition {
    pub pos: Vec2<i32>,
}

#[derive(Component)]
pub struct WorldPosition {
    pub pos: Vec2<i32>,
}

#[derive(Component)]
pub struct Sprite {
    pub section: Rect<u32>,
    pub anchor: Vec2<u32>,
    pub layer: u8,
}
