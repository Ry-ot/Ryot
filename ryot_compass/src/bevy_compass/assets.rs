use bevy::asset::Handle;
use bevy::prelude::{Image, Resource, TextureAtlas};
use bevy::utils::HashMap;
use bevy_asset_loader::asset_collection::AssetCollection;
use ryot::appearances::SpriteSheetDataSet;
use ryot::bevy_ryot::{Appearance, Catalog, ContentAssets, PreparedAppearances};
use ryot::prelude::*;

#[derive(Default)]
pub struct AtlasCollection {
    pub sheet_data_set: Option<SpriteSheetDataSet>,
    pub handles: HashMap<String, Handle<TextureAtlas>>,
}

#[derive(AssetCollection, Resource, Default)]
pub struct CompassContentAssets {
    atlas: AtlasCollection,

    // Config related handles
    #[asset(path = "appearances.dat")]
    appearances: Handle<Appearance>,
    #[asset(path = "catalog-content.json")]
    catalog_content: Handle<Catalog>,
    prepared_appearances: PreparedAppearances,

    // Image related handles
    #[asset(path = "ryot_mascot.png")]
    mascot: Handle<Image>,
    #[cfg(feature = "pre_loaded_sprites")]
    #[asset(path = "sprite-sheets", collection(typed, mapped))]
    sprite_sheets: HashMap<String, Handle<Image>>,
    #[cfg(not(feature = "pre_loaded_sprites"))]
    sprite_sheets: HashMap<String, Handle<Image>>,
}

impl ContentAssets for CompassContentAssets {}
impl AppearancesAssets for CompassContentAssets {
    fn appearances(&self) -> &Handle<Appearance> {
        &self.appearances
    }
    fn catalog_content(&self) -> &Handle<Catalog> {
        &self.catalog_content
    }

    fn prepared_appearances(&self) -> &PreparedAppearances {
        &self.prepared_appearances
    }

    fn prepared_appearances_mut(&mut self) -> &mut PreparedAppearances {
        &mut self.prepared_appearances
    }
}

impl SpriteAssets for CompassContentAssets {
    fn sprite_sheets(&self) -> &HashMap<String, Handle<Image>> {
        &self.sprite_sheets
    }

    fn sprite_sheet_data_set(&self) -> &Option<SpriteSheetDataSet> {
        &self.atlas.sheet_data_set
    }

    fn set_sprite_sheets_data(&mut self, sprite_sheet_set: SpriteSheetDataSet) {
        self.atlas.sheet_data_set.replace(sprite_sheet_set);
    }

    fn atlas_handles(&self) -> &HashMap<String, Handle<TextureAtlas>> {
        &self.atlas.handles
    }

    fn insert_atlas_handle(&mut self, file: &str, handle: Handle<TextureAtlas>) {
        self.atlas.handles.insert(file.to_string(), handle);
    }

    fn get_atlas_handle(&self, file: &str) -> Option<&Handle<TextureAtlas>> {
        self.atlas.handles.get(file)
    }
}

pub trait MascotAssets: Resource + AssetCollection + Send + Sync + 'static {
    fn mascot(&self) -> Handle<Image>;
}

impl MascotAssets for CompassContentAssets {
    fn mascot(&self) -> Handle<Image> {
        self.mascot.clone()
    }
}
