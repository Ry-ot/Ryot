use crate::appearances::{ContentType, SpriteSheetSet};
use crate::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;

pub struct ContentPlugin;

impl Plugin for ContentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Sprites>()
            .add_plugins(JsonAssetPlugin::<Catalog>::new(&["json"]));

        app.add_plugins(AppearanceAssetPlugin)
            .add_plugins(ConfigPlugin::<ContentConfigs>::default());

        app.add_loading_state(
            LoadingState::new(RyotSetupStates::LoadingContent)
                .continue_to_state(RyotSetupStates::PreparingContent)
                .load_collection::<ContentAssets>(),
        )
        .add_systems(OnEnter(RyotSetupStates::PreparingContent), prepare_content);
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
    pub appearances: Handle<crate::bevy_ryot::Appearance>,
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
    mut state: ResMut<NextState<RyotSetupStates>>,
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

    state.set(RyotSetupStates::LoadingSprites);

    info!("Finished preparing content");
}
