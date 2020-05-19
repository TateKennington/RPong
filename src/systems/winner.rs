use amethyst::{
    core::{transform::Transform, SystemDesc},
    derive::{SystemDesc},
    ecs::prelude::{Join, System, SystemData, World, WriteStorage, Write, ReadExpect, Read},
    ui::UiText,
    assets::AssetStorage,
    audio::{output::Output, Source},
};

use std::ops::Deref;

use crate::pong::{Ball, ARENA_WIDTH, ScoreBoard, ScoreText};
use crate::audio::{play_score_sound, Sounds};

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem{
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,
        ReadExpect<'s, ScoreText>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>
    );

    fn run(&mut self, (mut balls, mut transforms, mut ui_text, mut scores, score_text, storage, sounds, audio_output): Self::SystemData) {
        for (ball, transform) in (&mut balls, &mut transforms).join() {
            let ball_x = transform.translation().x;

            let did_hit = if ball_x <= ball.radius {
                scores.score_right = (scores.score_right + 1).min(999);
                if let Some(text) = ui_text.get_mut(score_text.p2_score) {
                    text.text = scores.score_right.to_string();
                }
                true
            } else if ball_x + ball.radius >= ARENA_WIDTH {
                scores.score_left = (scores.score_left + 1).min(999);
                if let Some(text) = ui_text.get_mut(score_text.p1_score) {
                    text.text = scores.score_left.to_string();
                }
                true
            } else {
                false
            };

            if did_hit{
                ball.velocity[0] *= -1.0;
                transform.set_translation_x(0.5 * ARENA_WIDTH);
                play_score_sound(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()))
            }
        }
    }
}