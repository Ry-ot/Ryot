#![feature(let_chains)]

pub mod item;

#[cfg(all(feature = "lmdb", not(target_arch = "wasm32")))]
pub mod lmdb;

#[cfg(all(feature = "lmdb", not(target_arch = "wasm32")))]
pub use lmdb::*;

mod generator;
pub use generator::{build_map, get_chunks_per_z};

mod plan;
pub use plan::*;

mod serde;
pub use serde::types::*;

mod error;
pub use error::*;

mod error_handling;
pub use error_handling::*;

pub mod helpers;

pub mod minimap;

mod ryot_bevy;
pub use ryot_bevy::*;

mod tileset;
pub use tileset::*;

mod ui;
pub use ui::*;

mod config;
pub use config::*;
