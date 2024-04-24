#![feature(trait_alias)]
pub mod catalog;
pub mod plugins;
pub mod sprites;
pub mod visual_elements;

pub mod prelude {
    pub use crate::{
        catalog::{prepare_sprite_sheets, Catalog, CatalogAsset},
        content_plugin,
        plugins::{BaseContentPlugin, MetaContentPlugin},
        sprites::{prepare_sprite_layouts, prepare_sprite_meshes},
        visual_elements::{prepare_visual_elements, VisualElementsAsset},
    };
}
