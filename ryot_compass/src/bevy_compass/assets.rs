use bevy::asset::Handle;
use bevy::prelude::{Image, Resource};
use bevy::utils::HashMap;
use bevy_asset_loader::asset_collection::AssetCollection;
use ryot::bevy_ryot::sprites::SpriteAssets;
use ryot::bevy_ryot::{Appearance, Catalog, ConfigAsset, ContentAssets};
use ryot::ContentConfigs;

#[derive(AssetCollection, Resource)]
pub struct CompassContentAssets {
    #[asset(path = "appearances.dat")]
    appearances: Handle<Appearance>,
    #[asset(path = "catalog-content.json")]
    catalog_content: Handle<Catalog>,
    #[asset(path = "config/.content.toml")]
    config: Handle<ConfigAsset<ContentConfigs>>,
}

impl ContentAssets for CompassContentAssets {
    fn appearances(&self) -> &Handle<Appearance> {
        &self.appearances
    }

    fn catalog_content(&self) -> &Handle<Catalog> {
        &self.catalog_content
    }

    fn config(&self) -> &Handle<ConfigAsset<ContentConfigs>> {
        &self.config
    }
}

#[derive(AssetCollection, Resource)]
pub struct CompassSpriteAssets {
    #[cfg(feature = "pre_loaded_sprites")]
    #[asset(path = "sprite-sheets", collection(typed, mapped))]
    pub sprite_sheets: HashMap<String, Handle<Image>>,
    #[cfg(not(feature = "pre_loaded_sprites"))]
    sprite_sheets: HashMap<String, Handle<Image>>,
    #[asset(path = "ryot_mascot.png")]
    pub mascot: Handle<Image>,
}

impl SpriteAssets for CompassSpriteAssets {
    fn sprite_sheets(&self) -> &HashMap<String, Handle<Image>> {
        &self.sprite_sheets
    }
}
