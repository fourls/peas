mod app;
mod components;
mod config;
mod constants;
mod resources;
mod spawn;
mod systems;
mod util;

fn main() {
    app::run().unwrap();
}
