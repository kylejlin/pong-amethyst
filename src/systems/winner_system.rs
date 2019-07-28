use crate::pong::{Ball, Score, ScoreText, ARENA_HEIGHT, ARENA_WIDTH};

use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadExpect, ReadStorage, System, Write, WriteStorage},
    ui::UiText,
};

pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        Write<'s, Score>,
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, ScoreText>,
    );

    fn run(
        &mut self,
        (mut score, balls, mut transforms, mut ui_text, score_text): Self::SystemData,
    ) {
        for (_, transform) in (&balls, &mut transforms).join() {
            let ball_x = transform.translation().x;
            let ball_z = transform.translation().z;

            if ball_x < 0.0.into() {
                score.increment_right();
                if let Some(text) = ui_text.get_mut(score_text.1) {
                    text.text = score.1.to_string();
                }
                transform.set_translation_xyz(0.5 * ARENA_WIDTH, 0.5 * ARENA_HEIGHT, ball_z);
            }
            if ball_x > ARENA_WIDTH.into() {
                score.increment_left();
                if let Some(text) = ui_text.get_mut(score_text.0) {
                    text.text = score.0.to_string();
                }
                transform.set_translation_xyz(0.5 * ARENA_WIDTH, 0.5 * ARENA_HEIGHT, ball_z);
            }
        }
    }
}