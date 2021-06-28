use crate::components::*;
use crate::spawn::{self, TileType};
use crate::systems;
use crow::glutin::dpi::LogicalSize;
use crow::glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::WindowBuilder,
};
use crow::{Context, DrawConfig, Texture};
use specs::prelude::*;

const CLEAR_COLOR: (f32, f32, f32, f32) = (0.7, 0.7, 0.7, 1.0);
const SCALE_FACTOR: u32 = 4;

fn setup_ecs() -> World {
    let mut world = World::new();

    world.register::<Sprite>();
    world.register::<Position>();
    world.register::<TilePosition>();

    spawn::player(&mut world, (0, 0));
    spawn::tile(&mut world, TileType::Grass, (0, 0));
    spawn::tile(&mut world, TileType::Grass, (0, 1));
    spawn::tile(&mut world, TileType::Cobble, (1, 0));
    spawn::tile(&mut world, TileType::Grass, (1, 1));
    spawn::tile(&mut world, TileType::Soil, (-1, 0));

    world
}

pub fn run() -> Result<(), crow::Error> {
    let event_loop = EventLoop::new();
    let mut ctx = Context::new(
        WindowBuilder::new().with_inner_size(LogicalSize::new(400, 400)),
        &event_loop,
    )?;
    let spritesheet = Texture::load(&mut ctx, "./assets/sprites.png")?;

    let mut ecs = setup_ecs();

    event_loop.run(
        move |event: Event<()>, _window_target: _, control_flow: &mut ControlFlow| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => ctx.window().request_redraw(),
            Event::RedrawRequested(_) => tick(&mut ctx, &spritesheet, &mut ecs),
            _ => {}
        },
    )
}

fn tick(ctx: &mut Context, spritesheet: &Texture, world: &mut World) {
    systems(world);

    world.maintain();

    draw(ctx, spritesheet, world);
}

fn draw(ctx: &mut Context, spritesheet: &Texture, world: &World) {
    let mut surf = ctx.surface();

    ctx.clear_color(&mut surf, CLEAR_COLOR);
    // ctx.draw(
    //     &mut surf,
    //     &spritesheet.get_section((32, 72), (64, 64)),
    //     (100, 150),
    //     &DrawConfig {
    //         scale: (SCALE_FACTOR, SCALE_FACTOR),
    //         ..Default::default()
    //     },
    // );

    let sprites = world.read_storage::<Sprite>();
    let positions = world.read_storage::<Position>();

    for (sprite, pos) in (&sprites, &positions).join() {
        let tex = spritesheet.get_section(sprite.section.pos(), sprite.section.size());

        let x = pos.x - sprite.anchor.0 as i32;
        let y = pos.y - sprite.anchor.1 as i32;

        ctx.draw(
            &mut surf,
            &tex,
            (x * SCALE_FACTOR as i32, y * SCALE_FACTOR as i32),
            &DrawConfig {
                scale: (SCALE_FACTOR, SCALE_FACTOR),
                depth: Some((u8::MAX - sprite.layer) as f32 / u8::MAX as f32),
                ..Default::default()
            },
        );
    }

    ctx.present(surf).unwrap();
}

fn systems(world: &mut World) {
    systems::TilePositionUpdateSystem {}.run_now(world);
}
