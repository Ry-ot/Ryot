#![feature(trait_alias)]
//! `ryot_utils`
//!
//! Provides general utilities and helpers that are fundamental across the Ryot framework.
//! This crate includes functions and structs that assist in various aspects of game development,
//! ensuring that core utilities are reusable and accessible.

#[cfg(feature = "bevy")]
pub mod app;
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
pub mod cooldown;
#[cfg(feature = "bevy")]
pub mod window;

pub mod prelude {
    pub use crate::{async_task::execute, is_true, ThreadSafe};

    #[cfg(feature = "bevy")]
    pub use crate::{
        app::{InitResourceOnce, OptionalPlugin},
        async_events::{AsyncEventApp, EventSender},
        cache::{Cache, CacheSystems, SimpleCache},
        conditions::{on_hold_every, run_every, run_every_millis, run_every_secs, TimeArg},
        cooldown::{is_valid_cooldown_for_entity, Cooldown, CooldownApp},
        on_hold_every,
        window::entitled_window,
    };

    #[cfg(feature = "compression")]
    pub use crate::compression::{compress, decompress, Compression, Zstd};
}

pub trait ThreadSafe = Send + Sync + 'static;

pub fn is_true(value: Option<bool>) -> bool {
    value == Some(true)
}
