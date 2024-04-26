use crate::prelude::*;
use bevy_time::{Timer, TimerMode};
use rand::Rng;
use ryot_core::prelude::Animation;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnimationKey {
    pub phase_durations: Vec<Duration>,
    pub start_phase: AnimationStartPhase,
    pub total_phases: usize,
}

impl AnimationKey {
    pub(crate) fn create_timer(&self, phase: usize) -> Timer {
        Timer::new(self.duration(phase), TimerMode::Once)
    }

    pub(crate) fn duration(&self, phase: usize) -> Duration {
        self.phase_durations
            .get(phase)
            .cloned()
            .unwrap_or(Duration::from_millis(300))
    }

    pub fn default_state(&self) -> AnimationState {
        let current_phase = self.start_phase.get(self.total_phases);
        AnimationState::new(current_phase, self.create_timer(current_phase))
    }
}

pub trait SpriteAnimationExt {
    fn get_animation_key(&self) -> AnimationKey;
}

impl SpriteAnimationExt for Animation {
    fn get_animation_key(&self) -> AnimationKey {
        let phase_durations = self
            .phases
            .iter()
            .map(|(min, max)| -> Duration {
                let range = *min..*max;
                if range.start == range.end {
                    return Duration::from_millis(range.start.into());
                }
                Duration::from_millis(rand::thread_rng().gen_range(range).into())
            })
            .collect();

        AnimationKey {
            phase_durations,
            start_phase: match self.is_start_random {
                true => AnimationStartPhase::Random,
                false => AnimationStartPhase::Fixed(self.start_phase as usize),
            },
            total_phases: self.phases.len(),
        }
    }
}
