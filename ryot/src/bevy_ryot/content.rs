//! Content plugin for loading content assets and preparing them for use in the game.
use crate::appearances::{ContentType, SpriteSheetSet};
use crate::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use std::marker::PhantomData;

/// A trait that represents the content assets of a game.
/// It expects the type to implement AssetCollection and Resource.
/// It's a Bevy resource that holds the handles to the assets loaded by bevy_asset_loader.
///
/// Assets contains appearances (loaded from a *.dat file), a catalog (loaded from a *.json file)
/// and a config (loaded from a *.toml file).
pub trait ContentAssets: Resource + AssetCollection + Send + Sync + 'static {
    fn appearances(&self) -> &Handle<Appearance>;
    fn catalog_content(&self) -> &Handle<Catalog>;
    fn config(&self) -> &Handle<ConfigAsset<ContentConfigs>>;
}

/// A plugin that registers implementations of ContentAssets and loads them.
/// It inits the necessary resources and adds the necessary systems and plugins to load
/// the content assets.
///
/// It also manages the loading state of the content assets and the lifecycle of the content.
pub struct ContentPlugin<T: ContentAssets> {
    _marker: PhantomData<T>,
}

impl<T: ContentAssets> ContentPlugin<T> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<T: ContentAssets> Default for ContentPlugin<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: ContentAssets + Default> Plugin for ContentPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_resource::<T>();
        app.init_resource::<Sprites>()
            .add_plugins(JsonAssetPlugin::<Catalog>::new(&["json"]));

        app.add_plugins(AppearanceAssetPlugin)
            .add_plugins(ConfigPlugin::<ContentConfigs>::default());

        app.add_loading_state(
            LoadingState::new(InternalContentState::LoadingContent)
                .continue_to_state(InternalContentState::PreparingContent)
                .load_collection::<T>(),
        )
        .add_systems(
            OnEnter(InternalContentState::PreparingContent),
            prepare_content::<T>,
        );
    }
}

/// A resource that holds the sprite sheets loaded from the content assets.
#[derive(Resource, Debug, Default)]
pub struct Sprites {
    pub sheets: Option<SpriteSheetSet>,
}

/// An asset that holds a collection of raw content configs.
#[derive(serde::Deserialize, Asset, TypePath)]
#[serde(transparent)]
pub struct Catalog {
    pub content: Vec<ContentType>,
}

/// A system that prepares the content assets for use in the game.
/// It transforms the raw content configs into sprite sheet sets and stores them in
/// a way that the game can use them.
///
/// This is the last step of the content loading process, triggering the sprite loading process.
fn prepare_content<T: ContentAssets>(
    contents: Res<Assets<Catalog>>,
    content_assets: Res<T>,
    configs: Res<Assets<ConfigAsset<ContentConfigs>>>,
    mut sprites: ResMut<Sprites>,
    mut state: ResMut<NextState<InternalContentState>>,
) {
    info!("Preparing content");
    let Some(ConfigAsset(configs)) = configs.get(content_assets.config().id()) else {
        panic!("No config found for content");
    };

    let Some(catalog) = contents.get(content_assets.catalog_content().id()) else {
        panic!("No catalog loaded");
    };

    sprites.sheets = Some(SpriteSheetSet::from_content(
        &catalog.content,
        &configs.sprite_sheet,
    ));

    state.set(InternalContentState::LoadingSprites);

    info!("Finished preparing content");
}
