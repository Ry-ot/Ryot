use bevy::asset::Handle;
use bevy::prelude::Resource;
use bevy_asset_loader::asset_collection::AssetCollection;
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
