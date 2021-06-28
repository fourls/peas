mod app;
mod camera;
mod components;
mod input;
mod spawn;
mod systems;
mod util;

pub const TILE_SIZE: u32 = 16;

fn main() {
    app::run().unwrap();
}
