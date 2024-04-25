use bevy_ecs::prelude::Res;
use bevy_time::prelude::{Time, Timer, TimerMode};
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::Actionlike;
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

/// A condition using leafwing-input-manager that runs the system if an action is pressed or if
/// the action is held down every X duration.
pub fn on_hold_every<A>(
    action: A,
    time_arg: TimeArg,
) -> impl FnMut(Res<ActionState<A>>, Res<Time>) -> bool
where
    A: Actionlike + Clone,
{
    let mut timer = Timer::new(time_arg.0, TimerMode::Repeating);

    move |action_state: Res<ActionState<A>>, time: Res<Time>| {
        timer.tick(time.delta());

        if action_state.just_pressed(&action) {
            timer.reset();
            return true;
        }

        action_state.pressed(&action) && timer.finished()
    }
}

/// A wrapper for Duration that allows you to define duration from different units.
/// It supports Duration, u64(ms), and &str (e.g. "1s", "100ms", "1us", "1ns").
pub struct TimeArg(Duration);

impl From<u64> for TimeArg {
    fn from(value: u64) -> Self {
        TimeArg(Duration::from_millis(value))
    }
}

impl From<Duration> for TimeArg {
    fn from(value: Duration) -> Self {
        TimeArg(value)
    }
}

impl From<&str> for TimeArg {
    fn from(value: &str) -> Self {
        let time_str = value;
        if time_str.ends_with("ms") {
            TimeArg(Duration::from_millis(
                time_str
                    .trim_end_matches("ms")
                    .parse()
                    .expect("Expected a number followed by 'ms'"),
            ))
        } else if time_str.ends_with("us") {
            TimeArg(Duration::from_micros(
                time_str
                    .trim_end_matches("us")
                    .parse()
                    .expect("Expected a number followed by 'us'"),
            ))
        } else if time_str.ends_with("ns") {
            TimeArg(Duration::from_nanos(
                time_str
                    .trim_end_matches("ns")
                    .parse()
                    .expect("Expected a number followed by 'ns'"),
            ))
        } else if time_str.ends_with('s') {
            TimeArg(Duration::from_secs(
                time_str
                    .trim_end_matches('s')
                    .parse()
                    .expect("Expected a number followed by 's'"),
            ))
        } else {
            panic!("Unsupported time unit. Supported units are 's', 'ms', 'us', 'ns'.");
        }
    }
}

/// A macro that simplifies the usege of `on_hold_every` condition by already calling
/// `into` on the time argument.
#[macro_export]
macro_rules! on_hold_every {
    ($func:expr, $action:expr, $time_arg:expr) => {
        $func.run_if(on_hold_every($action, $time_arg.into()))
    };
}
