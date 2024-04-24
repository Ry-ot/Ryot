//! Bevy plugins and utilities for RyOT games.
//!
//! This module is intended to be used as a library dependency for RyOT games.
//! It provides common ways of dealing with OT content, such as loading sprites,
//! configuring the game, and handling asynchronous events.
use self::sprites::SpriteMaterial;
#[cfg(feature = "debug")]
use crate::position::debug_sprite_position;
use crate::prelude::tibia::asset_loader::TibiaAssetsPlugin;
use crate::prelude::*;
use bevy::app::{App, Plugin, Update};
use bevy::asset::embedded_asset;
use bevy::prelude::*;
use bevy::sprite::Material2dPlugin;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_asset_loader::prelude::*;
use bevy_asset_loader::standard_dynamic_asset::StandardDynamicAssetArrayCollection;
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_stroked_text::StrokedTextPlugin;
use ryot_assets::prelude::*;
use ryot_tiled::prelude::*;
use std::marker::PhantomData;

mod game;
pub use game::*;

#[cfg(feature = "lmdb")]
pub mod lmdb;

pub mod drawing;

pub mod sprites;

pub(crate) mod sprite_animations;
pub use sprite_animations::{toggle_sprite_animation, AnimationDuration};

/// A plugin that registers implementations of ContentAssets and loads them.
/// It inits the necessary resources and adds the necessary systems and plugins to load
/// the content assets.
///
/// It also manages the loading state of the content assets, the lifecycle of the content
/// and the events that allow lazy loading of sprites.
macro_rules! content_plugin {
    ($plugin_name:ident, $content_assets:tt) => {
        pub struct $plugin_name<C: $content_assets>(PhantomData<C>);

        impl<C: $content_assets> Default for $plugin_name<C> {
            fn default() -> Self {
                Self(PhantomData)
            }
        }
    };
}

content_plugin!(BaseContentPlugin, VisualElementsAsset);

impl<C: VisualElementsAsset + Default> Plugin for BaseContentPlugin<C> {
    fn build(&self, app: &mut App) {
        app.init_state::<InternalContentState>()
            .init_resource::<C>()
            .init_resource::<VisualElements>()
            .register_type::<TilePosition>()
            .add_plugins(TibiaAssetsPlugin)
            .add_loading_state(
                LoadingState::new(InternalContentState::LoadingContent)
                    .continue_to_state(InternalContentState::PreparingContent)
                    .load_collection::<C>(),
            )
            .add_systems(
                OnEnter(InternalContentState::PreparingContent),
                prepare_visual_elements::<C>,
            );
    }
}

content_plugin!(MetaContentPlugin, VisualElementsAsset);

impl<C: VisualElementsAsset + Default> Plugin for MetaContentPlugin<C> {
    fn build(&self, app: &mut App) {
        app.add_plugins(BaseContentPlugin::<C>::default())
            .add_systems(
                OnEnter(InternalContentState::PreparingContent),
                transition_to_ready.after(prepare_visual_elements::<C>),
            );
    }
}

content_plugin!(VisualContentPlugin, VisualElementsAsset);

impl<C> Plugin for VisualContentPlugin<C>
where
    C: VisualElementsAsset + CatalogAsset + AtlasLayoutsAsset + Default,
{
    fn build(&self, app: &mut App) {
        app.add_plugins(BaseContentPlugin::<C>::default())
            .configure_loading_state(
                LoadingStateConfig::new(InternalContentState::LoadingContent)
                    .with_dynamic_assets_file::<StandardDynamicAssetArrayCollection>(
                    "dynamic.atlases.ron",
                ),
            )
            .init_resource::<RectMeshes>()
            .init_resource::<SpriteMeshes>()
            .init_resource::<SpriteSheetDataSet>()
            .init_resource::<TextureAtlasLayouts>()
            .add_plugins(JsonAssetPlugin::<Catalog>::new(&["json"]))
            .add_optional_plugin(StrokedTextPlugin)
            .add_plugins(Material2dPlugin::<SpriteMaterial>::default())
            .add_systems(
                OnEnter(InternalContentState::PreparingContent),
                (
                    prepare_sprite_layouts::<C>,
                    prepare_sprite_sheets::<C>,
                    prepare_sprite_meshes,
                    transition_to_ready,
                )
                    .chain()
                    .after(prepare_visual_elements::<C>),
            )
            .init_resource::<sprite_animations::SpriteAnimationEnabled>()
            .init_resource::<sprite_animations::SynchronizedAnimationTimers>()
            .init_resource::<sprites::LoadedAppearances>()
            .add_event::<sprites::LoadAppearanceEvent>()
            .add_systems(
                Update,
                (
                    #[cfg(feature = "debug")]
                    debug_sprite_position,
                    sprites::load_from_entities_system.in_set(SpriteSystems::Load),
                    sprites::process_load_events_system
                        .pipe(sprites::load_sprite_system)
                        .pipe(sprites::store_loaded_appearances_system)
                        .run_if(on_event::<sprites::LoadAppearanceEvent>())
                        .in_set(SpriteSystems::Load),
                    sprites::initialize_sprite_material.in_set(SpriteSystems::Initialize),
                    sprites::update_sprite_system.in_set(SpriteSystems::Update),
                    sprite_animations::initialize_animation_sprite_system
                        .in_set(AnimationSystems::Initialize),
                    sprite_animations::tick_animation_system
                        .run_if(resource_exists_and_equals(
                            sprite_animations::SpriteAnimationEnabled(true),
                        ))
                        .in_set(AnimationSystems::Update),
                )
                    .chain()
                    .run_if(in_state(InternalContentState::Ready)),
            )
            .add_systems(
                PostUpdate,
                (
                    update_sprite_position,
                    (move_sprites_with_animation, finish_position_animation).chain(),
                ),
            );

        embedded_asset!(app, "shaders/sprite.wgsl");
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum SpriteSystems {
    Load,
    Initialize,
    Update,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum AnimationSystems {
    Initialize,
    Update,
}
