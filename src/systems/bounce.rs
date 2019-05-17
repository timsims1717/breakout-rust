use amethyst::{
    core::{
        nalgebra::{Rotation3,Translation3,UnitQuaternion,Vector3},
        transform::Transform,
    },
    ecs::prelude::{Entities, Join, ReadStorage, System, WriteStorage},
};

use crate::breakout::{Ball, Paddle, Brick, Velocity, STAGE_WIDTH, STAGE_HEIGHT};
use crate::systems::bounce::BounceEdge::{LEFT, RIGHT, TOP, NONE, BOTTOM};

pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, Paddle>,
        WriteStorage<'s, Brick>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, mut balls, mut velocities, paddles, mut bricks, transforms): Self::SystemData) {
        // check whether a ball collided, and bounce off accordingly
        for (ball, velocity, transform) in (&mut balls, &mut velocities, &transforms).join() {
            let mut ball_collided = false;

            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            // bound at the top of the arena
            if ball_y >= STAGE_HEIGHT - ball.radius && velocity.y > 0.0 {
                velocity.y = -velocity.y;
                ball_collided = true;
            }

            // bound at the sides of the arena
            if ball_x >= STAGE_WIDTH - ball.radius && velocity.x > 0.0 {
                velocity.x = -velocity.x;
                ball_collided = true;
            } else if ball_x <= ball.radius && velocity.x < 0.0 {
                velocity.x = -velocity.x;
                ball_collided = true;
            }

            // bounce at the paddle
            for (paddle, paddle_transform) in (&paddles, &transforms).join() {
                // get corner coords of paddle
                let paddle_x = paddle_transform.translation().x - paddle.width * 0.5;

                // test if ball is in paddle hitbox
                if point_in_rect(
                    ball_x,
                    ball_y,
                    paddle_x - ball.radius,
                    paddle_transform.translation().y,
                    paddle_x + paddle.width + ball.radius,
                    paddle_transform.translation().y + paddle.height * 0.5 + ball.radius,
                ) && velocity.y < 0.0 {
                    // bounce on the paddle no angle
//                     let new_v_y = -velocity.y;
//                    velocity.y = new_v_y;

                    // bounce off at an angle
//                    let vel_vec = Vector3::new(velocity.x, velocity.y, 0.0.into());
                    let neut_vec = Vector3::new(0.0.into(), 80.0.into(), 0.0.into());
                    let ratio = 0.8 * (paddle_transform.translation().x - ball_x) / (paddle.width * 0.5);
                    let angle = ratio * std::f32::consts::FRAC_PI_2;
                    let rot = Rotation3::from_axis_angle(&Vector3::z_axis(), angle);
//                    let new_vel_vec = rot * vel_vec;
                    let new_vel_vec = rot * neut_vec;

                    velocity.x = new_vel_vec[0];
                    velocity.y = new_vel_vec[1];

                    ball_collided = true;
                }
            }

            // bounce at the bricks
            for (entity, brick, brick_transform) in (&*entities, &bricks, &transforms).join() {
                if ball_collided {
                    break;
                }
                // test if ball is in brick hitbox
                let e = in_brick_hitbox(
                    brick.width, brick.height,
                    brick_transform.translation().x,
                    brick_transform.translation().y,
                    ball.radius, ball_x, ball_y
                );
                if e == NONE {
                    continue;
                }
                // test if ball is going towards the brick
                if velocity.x < 0.0 && e == RIGHT
                    || velocity.x > 0.0 && e == LEFT {
                    velocity.x = -velocity.x;
                    ball_collided = true;
                    entities.delete(entity);
                }
                if velocity.y > 0.0 && e == BOTTOM
                    || velocity.y < 0.0 && e == TOP {
                    velocity.y = -velocity.y;
                    ball_collided = true;
                    entities.delete(entity);
                }
            }
        }
    }
}

// A point is in a box when its coordinates are smaller or equal than the top
// right and larger or equal than the bottom left
fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}

// A point is in a triangle if after solving the following equation system:
// p = p0 + (p1 - p0) * s + (p2 - p0) * t
// 0 <= s <= 1 and 0 <= t <= 1 and s + t <= 1
// s, t and 1 - s - t are called the barycentric coordinates of the point p
fn point_in_tri(x: f32, y: f32, p0x: f32, p0y: f32, p1x: f32, p1y: f32, p2x: f32, p2y: f32) -> bool {
    let area = 0.5 * (-p1y*p2x + p0y*(-p1x + p2x) + p0x*(p1y - p2y) + p1x*p2y);
    let s = 1.0 / (2.0*area) * (p0y*p2x - p0x*p2y + (p2y - p0y)*x + (p0x - p2x)*y);
    let t = 1.0 / (2.0*area) * (p0x*p1y - p0y*p1x + (p0y - p1y)*x + (p1x - p0x)*y);
    s >= 0.0 && t >= 0.0 && s + t <= 1.0
}

fn in_brick_hitbox(brick_width: f32, brick_height: f32, brick_x: f32, brick_y: f32, ball_r: f32, ball_x: f32, ball_y: f32) -> BounceEdge {
    // get corner coords of brick
    let half_brick_height = brick_height * 0.5;
    let brick_left = brick_x - brick_width * 0.5;
    let brick_bottom = brick_y - half_brick_height;
    let brick_right = brick_left + brick_width;
    let brick_top = brick_bottom + brick_height;

    // get hitbox coords
    let left = brick_left - ball_r;
    let bottom = brick_bottom - ball_r;
    let right = brick_right + ball_r;
    let top = brick_top + ball_r;

    if point_in_tri(
        ball_x, ball_y,
        left, top,
        left + half_brick_height, brick_y,
        left, bottom
    ) {
        LEFT
    } else if point_in_tri(
        ball_x, ball_y,
        right, top,
        right - half_brick_height, brick_y,
        right, bottom
    ) {
        RIGHT
    } else if point_in_rect(ball_x, ball_y, left, brick_y, right, top) {
        TOP
    } else if point_in_rect(ball_x, ball_y, left, bottom, right, brick_y) {
        BOTTOM
    } else {
        NONE
    }
}

#[derive(PartialEq, Eq)]
enum BounceEdge {
    LEFT,
    RIGHT,
    BOTTOM,
    TOP,
    NONE,
}