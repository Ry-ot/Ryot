use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::{not, Condition, IntoSystemConfigs, Res, Time, Timer, TimerMode};
use leafwing_input_manager::common_conditions::{action_just_pressed, action_pressed};
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

/// A helper for leafwing-input-manager that sets up a system to only run if a given action was pressed.
pub fn on_press<M, A: Actionlike>(systems: impl IntoSystemConfigs<M>, action: A) -> SystemConfigs {
    systems.run_if(action_just_pressed(action))
}

/// A helper for leafwing-input-manager that sets up a system to only run if a given action is being held.
pub fn on_hold<M, A: Actionlike>(systems: impl IntoSystemConfigs<M>, action: A) -> SystemConfigs {
    systems.run_if(action_pressed(action.clone()).and_then(not(action_just_pressed(action))))
}

pub fn on_hold_every<M, A: Actionlike>(
    systems: impl IntoSystemConfigs<M>,
    action: A,
    duration: Duration,
) -> SystemConfigs {
    systems.run_if(
        action_pressed(action.clone())
            .and_then(not(action_just_pressed(action)))
            .and_then(run_every(duration)),
    )
}

pub fn on_hold_every_millis<M, A: Actionlike>(
    systems: impl IntoSystemConfigs<M>,
    action: A,
    millis: u64,
) -> SystemConfigs {
    systems.run_if(
        action_pressed(action.clone())
            .and_then(not(action_just_pressed(action)))
            .and_then(run_every_millis(millis)),
    )
}

#[macro_export]
macro_rules! input_action {
    ($func:expr, $action:expr, $time:expr) => {
        (
            on_press($func, $action),
            on_hold_every_millis($func, $action, $time),
        )
    };
}
