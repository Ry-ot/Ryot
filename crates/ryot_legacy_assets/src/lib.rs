//! ryot_legacy_assets
//!
//! The `ryot_legacy_assets` crate provides a comprehensive suite of tools and utilities designed to
//! handle game assets derived from classic MMORPG frameworks. This crate is tailored to support
//! the specific needs of games inspired by or based on the Tibia engine, focusing on asset
//! management tasks that include:
//!
//!- **Decompression and Decoding**: Implements algorithms to decompress and decode sprite sheets,
//!     ensuring compatibility with legacy game formats.
//!- **Assets Handling**: Integrates with `appearances.proto` representations to manage and
//!     interpret game appearance definitions accurately.
//!- **Sprite Sheet Management**: Provides functionality to handle definitions and operations
//!     related to sprite sheets, facilitating the rendering and manipulation of game graphics.
//!- **Legacy Compatibility**: Ensures that modern implementations can seamlessly interact with
//!     traditional game asset structures, preserving the integrity and authenticity of the game's
//!     visual elements.
//!
//!### Purpose
//!
//! This crate aims to abstract the complexities of dealing with legacy game assets, providing a
//! robust API that enables developers to focus on game development without delving into the
//! intricacies of asset compatibility issues. By centralizing these functionalities.
//!
//! `ryot_legacy_assets` helps maintain clean and manageable codebases, enhances the scalability of
//! the game development process, and supports the integration of classic game elements into new
//! and evolving frameworks.
//!
//!### Usage Scenario
//!
//! Ideal for developers modernizing or building upon game engines from classic MMORPGs,
//! particularly those needing a reliable solution for managing and adapting old-style game assets.
//! This crate serves as a bridge, allowing the integration of traditional game assets into modern
//! game architecture efficiently and effectively.
//!
//! This is mostly focused on Tibia-like assets but could be improved to handle different assets.
pub mod appearances;

pub mod prelude {
    pub use crate::appearances::{prepared_appearances::PreparedAppearance, *};
}
