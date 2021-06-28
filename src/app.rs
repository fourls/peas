use crate::components::*;
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

    world
        .create_entity()
        .with(Sprite {
            offset: (32, 80),
            size: (16, 16),
            anchor: (8, 8),
        })
        .with(Position { x: 2, y: 2 })
        .build();

    world
}

pub fn run() -> Result<(), crow::Error> {
    let event_loop = EventLoop::new();
    let mut ctx = Context::new(WindowBuilder::new(), &event_loop)?;
    let spritesheet = Texture::load(&mut ctx, "./assets/sprites.png")?;

    let ecs = setup_ecs();

    event_loop.run(
        move |event: Event<()>, _window_target: _, control_flow: &mut ControlFlow| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => ctx.window().request_redraw(),
            Event::RedrawRequested(_) => draw(&mut ctx, &spritesheet, &ecs),
            _ => {}
        },
    )
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
        let tex = spritesheet.get_section(sprite.offset, sprite.size);

        let x = pos.x + sprite.anchor.0 as i32;
        let y = pos.y + sprite.anchor.1 as i32;

        ctx.draw(
            &mut surf,
            &tex,
            (x * SCALE_FACTOR as i32, y * SCALE_FACTOR as i32),
            &DrawConfig {
                scale: (SCALE_FACTOR, SCALE_FACTOR),
                ..Default::default()
            },
        );
    }

    ctx.present(surf).unwrap();
}
