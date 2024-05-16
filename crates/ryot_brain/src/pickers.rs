use bevy::prelude::*;
use big_brain::choices::Choice;
use big_brain::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct HighestWithThreshold {
    pub threshold: f32,
}

impl HighestWithThreshold {
    pub const ZERO_OR_MORE: Self = Self::new(0.0);

    pub const fn new(threshold: f32) -> Self {
        Self { threshold }
    }
}

impl Picker for HighestWithThreshold {
    fn pick<'a>(&self, choices: &'a [Choice], scores: &Query<&Score>) -> Option<&'a Choice> {
        Highest.pick(choices, scores).and_then(|choice| {
            if choice.calculate(scores) >= self.threshold {
                Some(choice)
            } else {
                None
            }
        })
    }
}
