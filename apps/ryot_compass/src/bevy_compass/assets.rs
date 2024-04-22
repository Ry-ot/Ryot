use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use ryot::bevy_ryot::Catalog;
use ryot::prelude::*;

#[derive(AssetCollection, Resource, Default)]
pub struct CompassContentAssets {
    #[asset(key = "layouts", collection(typed))]
    atlas_layout: Vec<Handle<TextureAtlasLayout>>,

    // Config related handles
    #[asset(path = "appearances.dat")]
    _appearances: Handle<VisualElements>,
    #[asset(path = "catalog-content.json")]
    catalog_content: Handle<Catalog>,

    // Image related handles
    #[asset(path = "ryot_mascot.png")]
    pub mascot: Handle<Image>,
}

impl PreloadedContentAssets for CompassContentAssets {
    fn catalog_content(&self) -> Handle<Catalog> {
        self.catalog_content.clone_weak()
    }
    fn get_atlas_layout(&self, layout: SpriteLayout) -> Option<Handle<TextureAtlasLayout>> {
        self.atlas_layout.get(layout as usize).cloned()
    }
}
