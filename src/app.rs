use std::collections::HashMap;

use crate::camera::Camera;
use crate::components::*;
use crate::spawn::{self, TileType};
use crate::systems;
use crate::util::Vec2;
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
const CAMERA_WIDTH: u32 = 256;
const CAMERA_HEIGHT: u32 = 192;

fn setup_ecs() -> World {
    let mut world = World::new();

    let camera = Camera {
        pos: Vec2::new(-(CAMERA_WIDTH as i32) / 2, -(CAMERA_HEIGHT as i32) / 2),
        width: CAMERA_WIDTH,
        height: CAMERA_HEIGHT,
    };

    world.register::<Sprite>();
    world.register::<ScreenPosition>();
    world.register::<WorldPosition>();

    spawn::player(&mut world, (0, 0).into());

    const BOUNDS: i32 = 2;

    let mut tile_overrides: HashMap<Vec2<i32>, TileType> = HashMap::new();
    tile_overrides.insert(Vec2::new(2, 0), TileType::Cobble);
    tile_overrides.insert(Vec2::new(2, 1), TileType::Cobble);
    tile_overrides.insert(Vec2::new(2, 2), TileType::Cobble);
    tile_overrides.insert(Vec2::new(1, 2), TileType::Cobble);
    tile_overrides.insert(Vec2::new(-1, 0), TileType::Water);
    tile_overrides.insert(Vec2::new(-1, 1), TileType::Water);
    tile_overrides.insert(Vec2::new(0, -1), TileType::Soil);
    tile_overrides.insert(Vec2::new(-1, -1), TileType::Soil);
    tile_overrides.insert(Vec2::new(1, -1), TileType::Soil);

    for x in -BOUNDS..=BOUNDS {
        for y in -BOUNDS..=BOUNDS {
            let pos = Vec2::new(x, y);

            spawn::tile(
                &mut world,
                match tile_overrides.get(&pos) {
                    None => TileType::Grass,
                    Some(tile_type) => tile_type.clone(),
                },
                camera.tile_to_world(&pos),
            );
        }
    }

    world.insert::<Camera>(camera);

    world
}

pub fn run() -> Result<(), crow::Error> {
    let event_loop = EventLoop::new();
    let mut ctx = Context::new(
        WindowBuilder::new().with_inner_size(LogicalSize::new(
            CAMERA_WIDTH * SCALE_FACTOR,
            CAMERA_HEIGHT * SCALE_FACTOR,
        )),
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

    let camera = world.read_resource::<Camera>();
    let sprites = world.read_storage::<Sprite>();
    let screen_positions = world.read_storage::<ScreenPosition>();

    for (sprite, pos) in (&sprites, &screen_positions).join() {
        let tex = spritesheet.get_section(sprite.section.pos(), sprite.section.size());

        let cam_x = pos.pos.x - sprite.anchor.x as i32 + camera.pos.x;
        let cam_y = pos.pos.y - sprite.anchor.y as i32 + camera.pos.y;

        ctx.draw(
            &mut surf,
            &tex,
            (cam_x * SCALE_FACTOR as i32, cam_y * SCALE_FACTOR as i32),
            &DrawConfig {
                scale: (SCALE_FACTOR, SCALE_FACTOR),
                depth: Some((u8::MAX - sprite.layer) as f32 / u8::MAX as f32),
                ..Default::default()
            },
        );
    }

    let world_positions = world.read_storage::<WorldPosition>();

    for (sprite, pos) in (&sprites, &world_positions).join() {
        let tex = spritesheet.get_section(sprite.section.pos(), sprite.section.size());

        let cam_pos = camera.screen_to_view(
            &camera.world_to_screen(&Vec2::new(pos.pos.x as i32, pos.pos.y as i32)),
        );

        ctx.draw(
            &mut surf,
            &tex,
            (
                (cam_pos.x - sprite.anchor.x as i32) * SCALE_FACTOR as i32,
                (cam_pos.y - sprite.anchor.y as i32) * SCALE_FACTOR as i32,
            ),
            &DrawConfig {
                scale: (SCALE_FACTOR, SCALE_FACTOR),
                depth: Some((u8::MAX - sprite.layer) as f32 / u8::MAX as f32),
                ..Default::default()
            },
        );
    }

    ctx.present(surf).unwrap();
}

fn systems(world: &mut World) {}
