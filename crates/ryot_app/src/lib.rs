//! `ryot_app`
//!
//! This crate contains essential plugins and bundles for building applications using
//! the Ryot framework. It facilitates the integration and management of Bevy engine
//! functionalities, streamlining game development.
pub mod content;
pub mod game;
#[cfg(feature = "lmdb")]
pub mod lmdb;
pub mod sprites;

pub mod prelude {
    pub use crate::{
        content::{BaseContentPlugin, MetaContentPlugin, VisualContentPlugin},
        content_plugin,
        game::{ElevationPlugin, GamePlugin, TileFlagPlugin},
        sprites::{RyotDrawingPlugin, RyotSpritePlugin},
    };

    #[cfg(feature = "lmdb")]
    pub use crate::lmdb::LmdbPlugin;

    #[cfg(feature = "tibia")]
    pub use ryot_tibia::prelude::TibiaAssetsPlugin;
}
