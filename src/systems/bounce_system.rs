use crate::audio::{self, Sounds};
use crate::pong::{Ball, Paddle, Side, ARENA_HEIGHT};

use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::{Float, Transform},
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
};

use std::ops::Deref;

pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, Transform>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(&mut self, (mut balls, paddles, transforms, storage, sounds, audio_output): Self::SystemData) {
        for (ball, transform) in (&mut balls, &transforms).join() {
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            if (ball_y.as_f32() < ball.radius) || (ball_y.as_f32() > ARENA_HEIGHT - ball.radius) {
                ball.velocity[1] *= -1.0;
                audio::play_bounce(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));
            }

            for (paddle, paddle_transform) in (&paddles, &transforms).join() {
                let paddle_x = paddle_transform.translation().x - Float::from(0.5 * paddle.width);
                let paddle_y = paddle_transform.translation().y - Float::from(0.5 * paddle.height);

                if point_in_rect(
                    ball_x,
                    ball_y,
                    paddle_x - ball.radius.into(),
                    paddle_y - ball.radius.into(),
                    paddle_x + paddle.width.into() + ball.radius.into(),
                    paddle_y + paddle.height.into() + ball.radius.into(),
                ) && ((paddle.side == Side::Left && ball.velocity[0] < 0.0)
                    || (paddle.side == Side::Right && ball.velocity[0] > 0.0))
                {
                    ball.velocity[0] *= -1.0;
                    audio::play_bounce(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));
                }
            }
        }
    }
}

fn point_in_rect(x: Float, y: Float, left: Float, bottom: Float, right: Float, top: Float) -> bool {
    left < x && x < right && bottom < y && y < top
}