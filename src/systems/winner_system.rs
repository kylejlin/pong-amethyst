use crate::pong::{Ball, Score, ARENA_HEIGHT, ARENA_WIDTH};

use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadStorage, System, Write, WriteStorage},
};

pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        Write<'s, Score>,
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (mut score, balls, mut transforms): Self::SystemData) {
        for (_, transform) in (&balls, &mut transforms).join() {
            let ball_x = transform.translation().x;
            let ball_z = transform.translation().z;
            if ball_x < 0.0.into() || ball_x > ARENA_WIDTH.into() {
                if ball_x < 0.0.into() {
                    score.increment_right();
                } else {
                    score.increment_left();
                }
                transform.set_translation_xyz(0.5 * ARENA_WIDTH, 0.5 * ARENA_HEIGHT, ball_z);
            }
        }
    }
}