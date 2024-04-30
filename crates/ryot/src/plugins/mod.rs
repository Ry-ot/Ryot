//! `plugins`
//!
//! This module contains essential plugins and bundles for building applications using
//! the Ryot framework. It facilitates the integration and management of Bevy engine
//! functionalities, streamlining game development.
pub mod content;
pub mod game;
#[cfg(feature = "lmdb")]
pub mod lmdb;
pub mod sprites;

pub mod prelude {
    pub use crate::{
        content_plugin,
        plugins::{
            content::{BaseContentPlugin, MetaContentPlugin, VisualContentPlugin},
            game::{ElevationPlugin, GamePlugin, NavigablePlugin},
            sprites::{RyotDrawingPlugin, RyotSpritePlugin},
        },
    };

    #[cfg(feature = "lmdb")]
    pub use crate::plugins::lmdb::LmdbPlugin;
}

pub use prelude::*;
