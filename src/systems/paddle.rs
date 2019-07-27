use crate::pong::{Paddle, Side, ARENA_HEIGHT, PADDLE_HEIGHT};

use amethyst::{
    core::{math::RealField, Float, Transform},
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            };
            if let Some(movement) = movement {
                if movement != 0.0 {
                    let scaled_movement = Float::from(movement * MOVEMENT_SPEED);
                    let old_paddle_y = transform.translation().y;
                    let new_paddle_y = old_paddle_y + scaled_movement;
                    let clamped_paddle_y = new_paddle_y
                        .min(Float::from(MAX_PADDLE_Y))
                        .max(Float::from(MIN_PADDLE_Y));
                    transform.set_translation_y(clamped_paddle_y);
                }
            }
        }
    }
}

const MOVEMENT_SPEED: f64 = 1.2;
const MIN_PADDLE_Y: f64 = 0.5 * PADDLE_HEIGHT as f64;
const MAX_PADDLE_Y: f64 = ARENA_HEIGHT as f64 - 0.5 * PADDLE_HEIGHT as f64;