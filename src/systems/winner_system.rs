use crate::audio::{self, Sounds};
use crate::pong::{Ball, Score, ScoreText, ARENA_HEIGHT, ARENA_WIDTH};

use amethyst::{
    assets::{AssetStorage},
    audio::{output::Output, Source},
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, Write, WriteStorage},
    ui::UiText,
};

use std::ops::Deref;

pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        Write<'s, Score>,
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, ScoreText>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(
        &mut self,
        (mut score, balls, mut transforms, mut ui_text, score_text, storage, sounds, audio_output): Self::SystemData,
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
                audio::play_score(&*sounds, &*storage, audio_output.as_ref().map(|o| o.deref()));
            }
            if ball_x > ARENA_WIDTH.into() {
                score.increment_left();
                if let Some(text) = ui_text.get_mut(score_text.0) {
                    text.text = score.0.to_string();
                }
                transform.set_translation_xyz(0.5 * ARENA_WIDTH, 0.5 * ARENA_HEIGHT, ball_z);
                audio::play_score(&*sounds, &*storage, audio_output.as_ref().map(|o| o.deref()));
            }
        }
    }
}