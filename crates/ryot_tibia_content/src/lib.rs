//! # Tibia Appearance Protocol Definitions
//!
//! This crate encapsulates the protocol definitions for Tibia's appearance data, providing
//! a structured interface to Tibia appearance assets. It primarily consists of automatically
//! generated Rust code from protocol buffer definitions, ensuring type safety and easy integration
//! with Rust codebases.
//!
//! ## Structure
//! - **Protocol Definitions**: The core definitions are generated from Tibia's `.proto` files,
//!   providing Rust structs that match the protobuf specifications for appearances. This is done
//!   at build time, and the generated code is included directly in the crate.
//!
//! - **Conversions Module**: Contains implementations for converting raw Tibia appearance data
//!   into more usable internal formats. This submodule bridges the gap between raw protocol data
//!   and the application-specific data structures used within the Ryot system.
//!
//! ## Usage
//! This crate is used internally to decode appearance data received in the Tibia-specific
//! format. It allows the Ryot system to work directly with well-defined Rust structs instead of
//! handling raw binary data, simplifying data manipulation and integration tasks.
//!
//! ## Example
//! Here is an example of how you might use this crate to decode appearance data:
//!
//!```rust
//! use ryot_content::prelude::VisualElements;
//! use ryot_tibia_content as tibia;
//!
//! // Vec::new() simulates raw bytes received from Tibia's appearance data
//! let visual_elements: VisualElements = tibia::from_bytes(&Vec::new()).unwrap();
//! ```
//!
//! ## Build
//! The actual `.proto` definitions and the generation of Rust code from these definitions are
//! handled in the crate build script, if the ci_assets feature is enabled. This ensures that the
//! generated code is always up-to-date and in sync with the latest Tibia appearance definitions.
//!
//! ## Catalog:
//! Tibia assets are stored in a content-catalog.json file that lists the resources the client needs.
//! There are 5 known types of tibia content: appearances, staticdata, staticmapdata, map and sprite.
//! We only use the sprites and appearances in this library.
//!
//! ```json
//! [
//!    {
//!      "type": "appearances",
//!      "file": "appearances.dat"
//!    },
//!    {
//!      "type": "staticdata",
//!      "file": "staticdata.dat"
//!    },
//!    {
//!      "type": "staticmapdata",
//!      "file": "staticmapdata.dat"
//!    },
//!    {
//!      "type": "map",
//!      "file": "map.otbm"
//!    },
//!    {
//!       "type": "sprite",
//!       "file": "spritesheet.png",
//!       "spritetype": 0,
//!       "firstspriteid": 100,
//!       "lastspriteid": 200,
//!       "area": 64
//!     }
//! ]
//! ```
use prost::{DecodeError, Message};
use ryot_content::prelude as ryot;

include!(concat!(env!("OUT_DIR"), "/tibia.rs"));

#[cfg(feature = "bevy")]
pub mod asset_loader;
pub mod conversions;

pub fn from_bytes(bytes: &[u8]) -> Result<ryot::VisualElements, DecodeError> {
    let visual_elements: VisualElements = VisualElements::decode(bytes)?;
    Ok(visual_elements.into())
}

pub mod prelude {
    pub use crate::{asset_loader::TibiaAssetsPlugin, conversions, *};
}
