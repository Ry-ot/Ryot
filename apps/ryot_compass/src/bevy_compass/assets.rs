use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::asset_collection::AssetCollection;
use ryot::appearances::SpriteSheetDataSet;
use ryot::bevy_ryot::{Appearance, Catalog, ContentAssets, PreparedAppearances};
use ryot::prelude::*;

#[derive(AssetCollection, Resource, Default)]
pub struct CompassContentAssets {
    #[asset(key = "layouts", collection(typed))]
    atlas_layout: Vec<Handle<TextureAtlasLayout>>,

    sheet_data_set: Option<SpriteSheetDataSet>,

    // Config related handles
    #[asset(path = "appearances.dat")]
    appearances: Handle<Appearance>,
    #[asset(path = "catalog-content.json")]
    catalog_content: Handle<Catalog>,
    prepared_appearances: PreparedAppearances,

    // Image related handles
    #[asset(path = "ryot_mascot.png")]
    mascot: Handle<Image>,
    sprite_sheets: HashMap<String, Handle<Image>>,
}

impl PreloadedContentAssets for CompassContentAssets {}

impl PreloadedAssets for CompassContentAssets {
    fn catalog_content(&self) -> Handle<Catalog> {
        self.catalog_content.clone_weak()
    }

    fn set_sprite_sheets_data(&mut self, sprite_sheet_set: SpriteSheetDataSet) {
        self.sheet_data_set.replace(sprite_sheet_set);
    }
}

impl ContentAssets for CompassContentAssets {
    fn sprite_sheet_data_set(&self) -> Option<&SpriteSheetDataSet> {
        self.sheet_data_set.as_ref()
    }

    fn get_texture(&self, file: &str) -> Option<Handle<Image>> {
        Some(self.sprite_sheets.get(file)?.clone_weak())
    }

    fn get_atlas_layout(&self, layout: SpriteLayout) -> Option<Handle<TextureAtlasLayout>> {
        self.atlas_layout.get(layout as usize).cloned()
    }
}

impl AppearanceAssets for CompassContentAssets {
    fn appearances(&self) -> Handle<Appearance> {
        self.appearances.clone_weak()
    }

    fn prepared_appearances_mut(&mut self) -> &mut PreparedAppearances {
        &mut self.prepared_appearances
    }

    fn prepared_appearances(&self) -> &PreparedAppearances {
        &self.prepared_appearances
    }
}

pub trait CompassAssets: ContentAssets + AssetCollection {
    fn mascot(&self) -> Handle<Image>;
}

impl CompassAssets for CompassContentAssets {
    fn mascot(&self) -> Handle<Image> {
        self.mascot.clone()
    }
}
