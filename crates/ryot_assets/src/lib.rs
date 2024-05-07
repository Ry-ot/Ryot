//! `ryot_assets`
//!
//! Manages the loading and processing of game assets. The `ryot_assets` crate provides
//! powerful tools for efficient handling and manipulation of resources in a game development
//! environment.
#![feature(trait_alias)]

use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_ecs::prelude::Resource;
use ryot_utils::prelude::*;

pub mod atlas;
pub mod catalog;
pub mod sprites;
pub mod visual_elements;

pub trait RyotAsset = Resource + AssetCollection + ThreadSafe;

pub mod prelude {
    pub use crate::{
        atlas::AtlasLayoutsAsset,
        catalog::{prepare_sprite_sheets, Catalog, CatalogAsset},
        ryot_asset,
        sprites::{prepare_sprite_layouts, prepare_sprite_meshes},
        visual_elements::{prepare_visual_elements, VisualElementsAsset},
    };
}

/// In the Ryot ecosystem the main asset struct must implement AtlasLayoutsAsset, CatalogAsset, and
/// VisualElementsAsset. Those assets are consumed during the preloading phase of the Ryot
/// application and later on they are no longer needed.
///
/// They use AssetCollection to dynamically load the assets from path/keys using bevy asset loader.
/// This macro simplifies the implementation of this main asset struct by providing a default struct
/// that implements the required traits and default file/key names.
///
/// Example:
/// ```rust
///     use bevy_asset::Handle;
///     use bevy_asset_loader::prelude::*;
///     use bevy_ecs::prelude::*;
///     use bevy_sprite::*;
///     use ryot_assets::prelude::*;
///     use ryot_core::prelude::*;
///     ryot_asset!(SimpleAsset);
/// ```
/// and
/// ```rust
///     use bevy_asset::Handle;
///     use bevy_asset_loader::prelude::*;
///     use bevy_ecs::prelude::*;
///     use bevy_sprite::*;
///     use ryot_assets::prelude::*;
///     use ryot_core::prelude::*;
///
///     #[derive(AssetCollection, Resource, Default)]
///     pub struct SimpleAsset {
///         #[asset(key = "layouts", collection(typed))]
///         atlas_layout: Vec<Handle<TextureAtlasLayout>>,
///         #[asset(path = "appearances.dat")]
///         visual_elements: Handle<VisualElements>,
///         #[asset(path = "catalog-content.json")]
///         catalog_content: Handle<Catalog>,
///     }
/// ```
/// are equivalents.
///
/// You can also customise it towards your asset needs by providing custom file/key names or
/// additional fields.
///
/// Example:
/// ```rust
///     use bevy_render::texture::Image;
///     use bevy_asset::prelude::*;
///     use bevy_asset_loader::prelude::*;
///     use bevy_ecs::prelude::*;
///     use bevy_sprite::*;
///     use ryot_assets::prelude::*;
///     use ryot_core::prelude::*;
///
///     ryot_asset!(
///         SimpleAssetWithCustomFields,
///         {
///             #[asset(path = "ryot_mascot.png")]
///             pub mascot: Handle<Image>,
///         }
///     );
///
///     ryot_asset!(
///         AssetWithCustomFiles,
///         "custom_layouts",
///         "custom_visuals.dat",
///         "custom_catalog.json"
///     );
///
///     ryot_asset!(
///         CustomAsset,
///         "custom_layouts",
///         "custom_visuals.dat",
///         "custom_catalog.json",
///         {
///             #[asset(path = "ryot_mascot.png")]
///             pub mascot: Handle<Image>,
///         }
///     );
/// ```
/// and
/// ```rust
///     use bevy_render::texture::Image;
///     use bevy_asset::prelude::*;
///     use bevy_asset_loader::prelude::*;
///     use bevy_ecs::prelude::*;
///     use bevy_sprite::*;
///     use ryot_assets::prelude::*;
///     use ryot_core::prelude::*;
///
///     #[derive(AssetCollection, Resource, Default)]
///     pub struct SimpleAssetWithCustomFields {
///         #[asset(key = "layouts", collection(typed))]
///         atlas_layout: Vec<Handle<TextureAtlasLayout>>,
///         #[asset(path = "appearances.dat")]
///         visual_elements: Handle<VisualElements>,
///         #[asset(path = "catalog-content.json")]
///         catalog_content: Handle<Catalog>,
///         #[asset(path = "ryot_mascot.png")]
///         pub mascot: Handle<Image>,
///     }
///
///     #[derive(AssetCollection, Resource, Default)]
///     pub struct AssetWithCustomFiles {
///         #[asset(key = "layouts", collection(typed))]
///         atlas_layout: Vec<Handle<TextureAtlasLayout>>,
///         #[asset(path = "custom_layouts")]
///         visual_elements: Handle<VisualElements>,
///         #[asset(path = "custom_visuals.dat")]
///         catalog_content: Handle<Catalog>,
///     }
///
///     #[derive(AssetCollection, Resource, Default)]
///     pub struct CustomAsset {
///         #[asset(key = "layouts", collection(typed))]
///         atlas_layout: Vec<Handle<TextureAtlasLayout>>,
///         #[asset(path = "custom_layouts")]
///         visual_elements: Handle<VisualElements>,
///         #[asset(path = "custom_visuals.dat")]
///         catalog_content: Handle<Catalog>,
///         #[asset(path = "custom_catalog.json")]
///         pub mascot: Handle<Image>,
///     }
/// ```
/// are equivalents.
///
/// The trait implementation is abstracted away by
#[macro_export]
macro_rules! ryot_asset {
    ($name:ident) => {
        ryot_asset!(
            $name,
            {}
        );
    };

    ($name:ident, {$($custom:tt)*}) => {
        ryot_asset!(
            $name,
            "layouts",
            "appearances.dat",
            "catalog-content.json",
            {$($custom)*}
        );
    };

    ($name:ident, $layout_key:tt, $visual_elements_path:tt, $catalog_content_path:tt) => {
        ryot_asset!(
            $name,
            $layout_key,
            $visual_elements_path,
            $catalog_content_path,
            {}
        );
    };

    ($name:ident, $layout_key:tt, $visual_elements_path:tt, $catalog_content_path:tt, {$($custom:tt)*}) => {
        #[derive(AssetCollection, Resource, Default)]
        pub struct $name {
            #[asset(key = $layout_key, collection(typed))]
            atlas_layout: Vec<Handle<TextureAtlasLayout>>,
            #[asset(path = $visual_elements_path)]
            visual_elements: Handle<VisualElements>,
            #[asset(path = $catalog_content_path)]
            catalog_content: Handle<Catalog>,
            $($custom)*
        }

        impl CatalogAsset for $name {
            fn catalog_content(&self) -> &Handle<Catalog> {
                &self.catalog_content
            }
        }

        impl VisualElementsAsset for $name {
            fn visual_elements(&self) -> &Handle<VisualElements> {
                &self.visual_elements
            }
        }

        impl AtlasLayoutsAsset for $name {
            fn atlas_layouts(&self) -> &Vec<Handle<TextureAtlasLayout>> {
                &self.atlas_layout
            }
        }
    };
}
