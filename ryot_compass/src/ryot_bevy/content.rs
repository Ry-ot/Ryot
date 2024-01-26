use crate::{AppStates, Appearance, AppearanceAssetPlugin, ConfigAsset, ConfigPlugin};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use ryot::prelude::ContentConfigs;
use ryot::prelude::{ContentType, SpriteSheetSet};

pub struct ContentPlugin;

impl Plugin for ContentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Sprites>()
            .add_plugins(JsonAssetPlugin::<Catalog>::new(&["json"]));

        app.add_plugins(AppearanceAssetPlugin)
            .add_plugins(ConfigPlugin::<ContentConfigs>::default());

        app.add_loading_state(
            LoadingState::new(AppStates::LoadingContent)
                .continue_to_state(AppStates::PreparingContent)
                .load_collection::<ContentAssets>(),
        )
        .add_systems(OnEnter(AppStates::PreparingContent), prepare_content);
    }
}

#[derive(Resource, Debug, Default)]
pub struct Sprites {
    pub sheets: Option<SpriteSheetSet>,
}

#[derive(serde::Deserialize, Asset, TypePath)]
#[serde(transparent)]
pub struct Catalog {
    pub content: Vec<ContentType>,
}

#[derive(AssetCollection, Resource)]
pub struct ContentAssets {
    #[asset(path = "appearances.dat")]
    pub appearances: Handle<Appearance>,
    #[asset(path = "catalog-content.json")]
    pub catalog_content: Handle<Catalog>,
    #[asset(path = "config/.content.toml")]
    pub config: Handle<ConfigAsset<ContentConfigs>>,
}

fn prepare_content(
    contents: Res<Assets<Catalog>>,
    static_assets: Res<ContentAssets>,
    configs: Res<Assets<ConfigAsset<ContentConfigs>>>,
    mut sprites: ResMut<Sprites>,
    mut state: ResMut<NextState<AppStates>>,
) {
    info!("Preparing content");
    let Some(ConfigAsset(configs)) = configs.get(static_assets.config.id()) else {
        panic!("No config found for content");
    };

    let Some(catalog) = contents.get(static_assets.catalog_content.id()) else {
        panic!("No catalog loaded");
    };

    sprites.sheets = Some(SpriteSheetSet::from_content(
        &catalog.content,
        &configs.sprite_sheet,
    ));

    state.set(AppStates::LoadingSprites);

    info!("Finished preparing content");
}
