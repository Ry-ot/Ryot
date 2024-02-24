use bevy::prelude::{Res, Time, Timer, TimerMode};
use std::time::Duration;

/// Many times we need to run a system with a cooldown, to avoid running it every frame.
/// Things like drawing, deleting, casting a spell, etc.
/// This function is a helper condition that only runs your system every X duration.
/// Keep in mind that the duration is in game time, not real world time.
///
/// For running in more specific time unit: [milliseconds](run_every_millis) and
/// [seconds](run_every_secs) are also available.
pub fn run_every(duration: Duration) -> impl FnMut(Res<Time>) -> bool {
    let mut timer = Timer::new(duration, TimerMode::Repeating);
    move |time: Res<Time>| timer.tick(time.delta()).finished()
}

pub fn run_every_millis(millis: u64) -> impl FnMut(Res<Time>) -> bool {
    run_every(Duration::from_millis(millis))
}

pub fn run_every_secs(secs: u64) -> impl FnMut(Res<Time>) -> bool {
    run_every(Duration::from_secs(secs))
}
