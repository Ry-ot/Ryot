//! `ryot`
//!
//! The `ryot` crate serves as the main entry point for the Ryot framework, providing
//! a unified API that aggregates all sub-crates. It simplifies the integration and
//! usage of the framework's extensive functionalities.
//!
//! This crate also contains essential plugins and bundles for building applications using
//! the Ryot framework. It facilitates the integration and management of Bevy engine
//! functionalities, streamlining game development.
pub mod plugins;

pub mod prelude {
    pub use crate::plugins::prelude::*;
    pub use ryot_internal::prelude::*;
}
