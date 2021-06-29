use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::{
    components::*,
    constants::*,
    resources::{Camera, Input},
    spawn::{self, TileType},
    systems,
    util::Vec2,
};
use crow::{
    glutin::{
        dpi::{LogicalPosition, LogicalSize},
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    Context, DrawConfig, Texture,
};
use specs::prelude::*;

fn setup_ecs() -> World {
    let mut world = World::new();

    let camera = Camera {
        pos: Vec2::new(-(VIEW_WIDTH as i32) / 2, -(VIEW_HEIGHT as i32) / 2),
        width: VIEW_WIDTH,
        height: VIEW_HEIGHT,
    };

    world.register::<Sprite>();
    world.register::<Player>();
    world.register::<ScreenPosition>();
    world.register::<WorldPosition>();
    world.register::<WorldCollider>();
    world.register::<GrowingCrop>();
    world.register::<WorldClickable>();
    world.register::<Item>();
    world.register::<PreventsMovement>();
    world.register::<InPlayerInventory>();
    world.register::<Velocity>();

    spawn::player(&mut world, Vec2::default());
    spawn::crop(&mut world, Vec2::new(0, -1));
    spawn::crop(&mut world, Vec2::new(-1, -1));
    spawn::crop(&mut world, Vec2::new(1, -1));

    const BOUNDS: i32 = 3;

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

            let tile_type = match tile_overrides.get(&pos) {
                None => {
                    if x.abs() == BOUNDS || y.abs() == BOUNDS {
                        TileType::Water
                    } else {
                        TileType::Grass
                    }
                }
                Some(tile_type) => tile_type.clone(),
            };

            spawn::tile(
                &mut world,
                tile_type,
                pos,
                match tile_type {
                    TileType::Water => true,
                    _ => false,
                },
            );
        }
    }

    world.insert::<Camera>(camera);
    world.insert::<Input>(Input::default());
    world.insert::<Duration>(Duration::default());

    world
}

pub fn run() -> Result<(), crow::Error> {
    let event_loop = EventLoop::new();
    let mut ctx = Context::new(
        WindowBuilder::new().with_inner_size(LogicalSize::new(
            VIEW_WIDTH * SCALE_FACTOR,
            VIEW_HEIGHT * SCALE_FACTOR,
        )),
        &event_loop,
    )?;
    let spritesheet = Texture::load(&mut ctx, "./assets/sprites.png")?;

    let mut last_frame = Instant::now();

    let mut ecs = setup_ecs();

    event_loop.run(
        move |event: Event<()>, _window_target: _, control_flow: &mut ControlFlow| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::WindowEvent { event, window_id } if window_id == ctx.window().id() => {
                match event {
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(key) = input.virtual_keycode {
                            let mut input_res = ecs.write_resource::<Input>();

                            input_res.process_keyboard_input(key, input.state);
                        }
                    }
                    WindowEvent::MouseInput { button, state, .. } => {
                        let mut input_res = ecs.write_resource::<Input>();

                        input_res.process_mouse_input(button, state);
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        let size =
                            LogicalPosition::<i32>::from_physical(position, SCALE_FACTOR as f64);

                        let mut input_res = ecs.write_resource::<Input>();
                        input_res.process_cursor_moved(size);
                    }
                    _ => {}
                }
            }
            Event::MainEventsCleared => ctx.window().request_redraw(),
            Event::RedrawRequested(_) => {
                {
                    let mut delta = ecs.write_resource::<Duration>();
                    let now = Instant::now();
                    *delta = now - last_frame;
                    last_frame = now;
                }

                tick(&mut ctx, &spritesheet, &mut ecs)
            }
            _ => {}
        },
    )
}

fn tick(ctx: &mut Context, spritesheet: &Texture, world: &mut World) {
    systems(world);

    world.write_resource::<Input>().frame_end();
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

        let cam_pos =
            camera.screen_to_view(&camera.world_to_screen(&Vec2::new(pos.pos.x, pos.pos.y)));

        let pos = Vec2::new(
            (cam_pos.x - sprite.anchor.x as f32) * SCALE_FACTOR as f32,
            (cam_pos.y - sprite.anchor.y as f32) * SCALE_FACTOR as f32,
        );

        let depth = depth_of(sprite, cam_pos.into());

        ctx.draw(
            &mut surf,
            &tex,
            (pos.x as i32, pos.y as i32),
            &DrawConfig {
                scale: (SCALE_FACTOR, SCALE_FACTOR),
                depth: Some(depth),
                ..Default::default()
            },
        );
    }

    ctx.present(surf).unwrap();
}

fn depth_of(sprite: &Sprite, pos: Vec2<f32>) -> f32 {
    let layer_norm = (10 - sprite.layer) as f32 / 10.;

    const Y_LIMIT: f32 = 512.0;

    let y_norm = pos.y.clamp(-Y_LIMIT, Y_LIMIT) / (2.0 * Y_LIMIT * 10.);

    layer_norm + y_norm
}

fn systems(world: &mut World) {
    systems::PlayerMovementSystem::default().run_now(world);
    systems::CropGrowthSystem::default().run_now(world);
    systems::CropHarvestSystem::default().run_now(world);
    systems::ItemPickupSystem::default().run_now(world);
    systems::ItemGravitateSystem::default().run_now(world);
    systems::VelocitySystem::default().run_now(world);
}
