use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::breakout::{Ball, Velocity};

pub struct MoveBallSystem;

impl<'s> System<'s> for MoveBallSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        ReadStorage<'s, Velocity>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (balls, velocities, mut locals, time): Self::SystemData) {
        // Move every ball according to its speed and the time passed
        for (ball, velocity, local) in (&balls, &velocities, &mut locals).join() {
            local.translate_x(velocity.x * time.delta_seconds());
            local.translate_y(velocity.y * time.delta_seconds());
        }
    }
}