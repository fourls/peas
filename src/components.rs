use specs::prelude::*;
use specs_derive::Component;

use crate::util::Rect;

#[derive(Component, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct TilePosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Sprite {
    pub section: Rect<u32>,
    pub anchor: (u32, u32),
    pub layer: u8,
}
