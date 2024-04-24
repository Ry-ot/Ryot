#![feature(trait_alias)]

use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_ecs::prelude::Resource;

pub mod atlas;
pub mod catalog;
pub mod plugins;
pub mod sprites;
pub mod visual_elements;

pub trait RyotAsset = Resource + AssetCollection + Send + Sync + 'static;

pub mod prelude {
    pub use crate::{
        atlas::AtlasLayoutsAsset,
        catalog::{prepare_sprite_sheets, Catalog, CatalogAsset},
        content_plugin,
        plugins::{BaseContentPlugin, MetaContentPlugin, VisualContentPlugin},
        sprites::{prepare_sprite_layouts, prepare_sprite_meshes},
        visual_elements::{prepare_visual_elements, VisualElementsAsset},
    };
}
