use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{
        Camera, PngFormat, Projection, SpriteRender, SpriteSheet,
        SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
    },
};
use crate::data::{
    load, Stage
};

pub struct Breakout;

pub const STAGE_WIDTH: f32 = 320.0;
pub const STAGE_HEIGHT: f32 = 180.0;
pub const PADDLE_WIDTH: f32 = 32.0;
pub const PADDLE_HEIGHT: f32 = 4.0;
pub const PADDLE_VELOCITY: f32 = 120.0;
pub const BALL_VELOCITY_X: f32 = 40.0;
pub const BALL_VELOCITY_Y: f32 = 70.0;
pub const MAX_VELOCITY: f32 = 150.0;
pub const BALL_RADIUS: f32 = 2.0;
pub const BRICK_WIDTH: f32 = 16.0;
pub const BRICK_HEIGHT: f32 = 6.0;
pub const BRICK_ZERO: usize = 0;
pub const BRICK_MAX: usize = 8;

impl SimpleState for Breakout {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet = load_sprite_sheet(world);
        let stage = load_stage(world);
        world.register::<Brick>(); // todo: remove
        initialize_paddle(world, sprite_sheet.clone());
        initialize_ball(world, sprite_sheet.clone());
        initialize_stage(world, stage, sprite_sheet);
        initialize_camera(world);
    }
}

// creates a basic orthographic projection
fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world.create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            STAGE_WIDTH,
            0.0,
            STAGE_HEIGHT,
        )))
        .with(transform)
        .build();
}

// creates a paddle centered on the bottom of the stage area
fn initialize_paddle(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let mut paddle_trans = Transform::default();
    // position the paddle
    let x = STAGE_WIDTH / 2.0;
    let y = 2.0;
    paddle_trans.set_xyz(x, y + PADDLE_HEIGHT, 0.0);

    // assign sprite
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 8,
    };

    // create the paddle
    world.create_entity()
        .with(sprite_render.clone())
        .with(Paddle{
            velocity: PADDLE_VELOCITY,
            height: PADDLE_HEIGHT,
            width: PADDLE_WIDTH,
        })
        .with(paddle_trans)
        .build();
}

fn initialize_ball(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    // position the ball
    let mut ball_trans = Transform::default();
    ball_trans.set_xyz(STAGE_WIDTH / 2.0, 6.0, 0.0);

    // assign sprite
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 9,
    };

    // create the ball
    world.create_entity()
        .with(sprite_render.clone())
        .with(Ball{radius: BALL_RADIUS})
        .with(Velocity{x: BALL_VELOCITY_X, y: BALL_VELOCITY_Y})
        .with(ball_trans)
        .build();
}

fn initialize_stage(world: &mut World, stage: Stage, sprite_sheet: SpriteSheetHandle) {
    let mut y = STAGE_HEIGHT - BRICK_HEIGHT * 0.5;
    for row in stage.bricks.iter() {
        let mut x = 0.0 + BRICK_WIDTH * 0.5;
        for brick in row.iter() {
            if *brick > BRICK_ZERO && *brick <= BRICK_MAX {
                // position the brick
                let mut b_trans = Transform::default();
                b_trans.set_xyz(x, y, 0.0);

                // assign sprite
                let sprite_render = SpriteRender {
                    sprite_sheet: sprite_sheet.clone(),
                    sprite_number: brick - 1,
                };

                // create brick
                world.create_entity()
                    .with(sprite_render.clone())
                    .with(Brick {
                        height: BRICK_HEIGHT,
                        width: BRICK_WIDTH,
                    })
                    .with(b_trans)
                    .build();
            }
            x += BRICK_WIDTH;
        }
        y -= BRICK_HEIGHT;
    }
}

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    // load the png
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "textures/breakout_sprites.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    // load the sprites
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "textures/breakout_sprites.ron",
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store,
    )
}

fn load_stage(world: &mut World) -> Stage {
    // load the stage
    load("test.stg".to_string())
}

pub struct Paddle {
    pub velocity: f32,
    pub width: f32,
    pub height: f32,
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

pub struct Ball {
    pub radius: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

pub struct Brick {
    pub width: f32,
    pub height: f32,
}

impl Component for Brick {
    type Storage = DenseVecStorage<Self>;
}

pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}