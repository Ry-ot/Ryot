//! `ryot_assets`
//!
//! Manages the loading and processing of game assets. The `ryot_assets` crate provides
//! powerful tools for efficient handling and manipulation of resources in a game development
//! environment.
#![feature(trait_alias)]

use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_ecs::prelude::Resource;

pub mod atlas;
pub mod catalog;
pub mod sprites;
pub mod visual_elements;

pub trait RyotAsset = Resource + AssetCollection + Send + Sync + 'static;

pub mod prelude {
    pub use crate::{
        atlas::AtlasLayoutsAsset,
        catalog::{prepare_sprite_sheets, Catalog, CatalogAsset},
        sprites::{prepare_sprite_layouts, prepare_sprite_meshes},
        visual_elements::{prepare_visual_elements, VisualElementsAsset},
    };
}
