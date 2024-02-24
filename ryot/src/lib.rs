//! MMORPG library based on the concepts of open tibia.
//!
//! Ryot is an event-driven library that provides simple utilities for building OT based games.
//! It is designed to be used with the [Bevy](https://bevyengine.org/) game engine.
//! It is currently in early development and is not yet ready for use.
//!
//! Ryot is design to integrate with OpenTibia concepts, facilitating the creation
//! of games that intend to use CIP-like contents/assets formats, as well as some
//! game mechanics.
//!
//! It provides a major components:
//! * [Content](crate::bevy_ryot::ContentAssets) - A collection of content assets that
//!   can be loaded into the game, including appearances.dat, catalog, sprites and configs.
//!
//! It also provides some utilities:
//! * [Appearance](crate::bevy_ryot::Appearance) - A collection of structs and utilities used to
//!   manipulate protobuf based appearances, including [Prost](https://docs.rs/prost-build/latest/prost_build/) generated structs
//!   from the appearances.proto file.
//! * [Bevy Helpers](crate::bevy_ryot) - A collection of helpers that can be used to send async events,
//!   load configurations, appearances, sprites and contents as BevyAssets.
//! * [Compression](crate::compression) - A compression utility that can be used to compress
//!   and decompress sprite sheets.
//! * [ContentBuilder](crate::build::content) - A builder that can be used to build
//!   content assets from the CIP client content folder, decompressing sprite sheets and
//!   copying the necessary files to the assets folder.
//! * [Sprite Utilities](crate::sprites) - Functions that can be used to decompress, manipulate
//!   and load sprite sheets as game assets, taking into considerations CIP-like sprite sheets
//!   structures.
//! * [Content Utilities](crate::content) - A collection of structs that can be used to manipulate
//!   contents, including configuring and loading them.
#![feature(fn_traits)]
#![feature(lazy_cell)]
#![feature(unboxed_closures)]

pub mod appearances;

#[cfg(feature = "bevy")]
pub mod bevy_ryot;

#[cfg(feature = "compression")]
mod compression;
#[cfg(feature = "compression")]
pub use compression::{compress, decompress, Compression, Zstd};

pub mod content;
pub use content::*;

#[cfg(feature = "lmdb")]
pub mod lmdb;

mod build;
pub mod sprites;

pub use sprites::*;

pub mod directional;
pub use directional::*;

pub mod helpers;

pub mod prelude {
    #[cfg(feature = "bevy")]
    pub use crate::bevy_ryot::*;
    pub use crate::build::*;
    #[cfg(feature = "compression")]
    pub use crate::compression::{compress, decompress, Compression, Zstd};
    pub use crate::content::*;
    pub use crate::directional::*;
    pub use crate::position::*;
    pub use crate::sprites::*;
}
