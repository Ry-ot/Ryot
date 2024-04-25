//! `ryot_utils`
//!
//! Provides general utilities and helpers that are fundamental across the Ryot framework.
//! This crate includes functions and structs that assist in various aspects of game development,
//! ensuring that core utilities are reusable and accessible.
#[cfg(feature = "bevy")]
pub mod async_events;
pub mod async_task;
#[cfg(feature = "bevy")]
pub mod cache;
#[cfg(feature = "compression")]
pub mod compression;

#[cfg(feature = "bevy")]
pub mod conditions;

#[cfg(feature = "bevy")]
pub mod plugins;

#[cfg(feature = "bevy")]
pub mod window;

pub mod prelude {
    pub use crate::{async_task::execute, is_true, Flag};

    #[cfg(feature = "bevy")]
    pub use crate::{
        async_events::{AsyncEventApp, EventSender},
        cache::{Cache, CacheSystems},
        conditions::{on_hold_every, run_every, run_every_millis, run_every_secs, TimeArg},
        on_hold_every,
        plugins::OptionalPlugin,
        window::entitled_window,
    };

    #[cfg(feature = "compression")]
    pub use crate::compression::{compress, decompress, Compression, Zstd};
}

pub fn is_true(value: Option<bool>) -> bool {
    value == Some(true)
}

// TODO: Temp til we figure out module structure
pub trait Flag: Copy + Clone + Eq + PartialEq + Default + Sync + Send + 'static {
    fn is_walkable(&self) -> bool;
    fn blocks_sight(&self) -> bool;
}
