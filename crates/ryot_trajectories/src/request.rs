use bevy_ecs::prelude::Component;
use std::time::Duration;

#[derive(Clone, Copy, Eq, PartialEq, Component, Debug)]
pub(crate) struct Params {
    pub max_hits: i32,
    pub reversed: bool,
    pub execution_type: ExecutionType,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            max_hits: 0,
            reversed: false,
            execution_type: ExecutionType::Once,
        }
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug)]
pub enum ExecutionType {
    #[default]
    Once,
    TimeBased(Duration),
}

impl ExecutionType {
    pub fn every_in_ms(ms: u64) -> Self {
        Self::TimeBased(Duration::from_millis(ms))
    }

    pub fn every_in_sec(secs: u64) -> Self {
        Self::TimeBased(Duration::from_secs(secs))
    }
}
