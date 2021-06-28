use specs::prelude::*;
use specs_derive::Component;

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Sprite {
    pub offset: (u32, u32),
    pub size: (u32, u32),
    pub anchor: (u32, u32),
}
