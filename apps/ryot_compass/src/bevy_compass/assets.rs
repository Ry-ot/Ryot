use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use ryot::bevy_ryot::Catalog;
use ryot::prelude::*;

#[derive(AssetCollection, Resource, Default)]
pub struct CompassContentAssets {
    #[asset(key = "layouts", collection(typed))]
    atlas_layout: Vec<Handle<TextureAtlasLayout>>,
    #[asset(path = "appearances.dat")]
    visual_elements: Handle<VisualElements>,
    #[asset(path = "catalog-content.json")]
    catalog_content: Handle<Catalog>,
    #[asset(path = "ryot_mascot.png")]
    pub mascot: Handle<Image>,
}

impl CatalogAsset for CompassContentAssets {
    fn catalog_content(&self) -> &Handle<Catalog> {
        &self.catalog_content
    }
}

impl VisualElementsAsset for CompassContentAssets {
    fn visual_elements(&self) -> &Handle<VisualElements> {
        &self.visual_elements
    }
}

impl AtlasLayoutsAsset for CompassContentAssets {
    fn atlas_layouts(&self) -> &Vec<Handle<TextureAtlasLayout>> {
        &self.atlas_layout
    }
}
