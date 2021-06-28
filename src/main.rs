mod app;
mod camera;
mod components;
mod input;
mod spawn;
mod systems;
mod util;

pub const TILE_SIZE: u32 = 16;
pub const PLAYER_SPEED: f32 = 2. * TILE_SIZE as f32;

fn main() {
    app::run().unwrap();
}
