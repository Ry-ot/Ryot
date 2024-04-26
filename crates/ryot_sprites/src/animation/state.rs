//! Sprite animations module.
use crate::prelude::*;
use bevy_time::*;
use rand::Rng;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationStartPhase {
    Random,
    Fixed(usize),
}

impl AnimationStartPhase {
    pub(crate) fn get(&self, total_phases: usize) -> usize {
        match self {
            AnimationStartPhase::Random => rand::thread_rng().gen_range(0..total_phases),
            AnimationStartPhase::Fixed(phase) => *phase,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnimationState {
    pub timer: Timer,
    pub current_phase: usize,
    just_finished: bool,
}

impl AnimationState {
    pub(crate) fn new(current_phase: usize, timer: Timer) -> Self {
        Self {
            timer,
            current_phase,
            just_finished: false,
        }
    }

    pub(crate) fn tick(&mut self, key: &AnimationKey, delta: Duration) {
        self.timer.tick(delta);
        self.just_finished = false;
        if self.timer.just_finished() {
            self.current_phase += 1;
            if self.current_phase >= key.total_phases {
                self.current_phase = 0;
            }
            self.timer.set_duration(key.duration(self.current_phase));
            self.timer.reset();
            self.just_finished = true;
        }
    }

    pub(crate) fn just_finished(&self) -> bool {
        self.just_finished
    }
}
