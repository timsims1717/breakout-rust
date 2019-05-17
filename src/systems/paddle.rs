use amethyst::{
    core::{
        Transform,
        timing::Time,
    },
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
};

use crate::breakout::{Paddle, STAGE_WIDTH, PADDLE_VELOCITY, PADDLE_WIDTH};

pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, Time>,
    );

    fn run (&mut self, (mut transforms, paddles, input, time): Self::SystemData) {
        for (_paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = input.axis_value("paddle");
            if let Some(mv_amount) = movement {
                if mv_amount != 0.0 {
                    let scaled_amount = PADDLE_VELOCITY * time.delta_seconds() * mv_amount as f32;
                    let paddle_x = transform.translation().x;
                    transform.set_x(
                        (paddle_x + scaled_amount)
                            .min(STAGE_WIDTH - PADDLE_WIDTH * 0.5)
                            .max(PADDLE_WIDTH * 0.5),
                    );
                }
            }
        }
    }
}