use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::asset_collection::AssetCollection;
use ryot::appearances::SpriteSheetDataSet;
use ryot::bevy_ryot::{Appearance, Catalog, ContentAssets, PreparedAppearances};
use ryot::prelude::*;

#[derive(AssetCollection, Resource, Default)]
pub struct CompassContentAssets {
    #[asset(texture_atlas_layout(tile_size_x = 32., tile_size_y = 32., columns = 12, rows = 12))]
    atlas_layout_32_32: Handle<TextureAtlasLayout>,

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
    #[cfg(feature = "pre_loaded_sprites")]
    #[asset(path = "sprite-sheets", collection(typed, mapped))]
    sprite_sheets: HashMap<String, Handle<Image>>,
    #[cfg(not(feature = "pre_loaded_sprites"))]
    sprite_sheets: HashMap<String, Handle<Image>>,
}

impl PreloadedContentAssets for CompassContentAssets {}

impl PreloadedAssets for CompassContentAssets {
    fn appearances(&self) -> Handle<Appearance> {
        self.appearances.clone_weak()
    }

    fn catalog_content(&self) -> Handle<Catalog> {
        self.catalog_content.clone_weak()
    }

    fn prepared_appearances_mut(&mut self) -> &mut PreparedAppearances {
        &mut self.prepared_appearances
    }

    fn sprite_sheets(&self) -> &HashMap<String, Handle<Image>> {
        &self.sprite_sheets
    }

    fn set_sprite_sheets_data(&mut self, sprite_sheet_set: SpriteSheetDataSet) {
        self.sheet_data_set.replace(sprite_sheet_set);
    }

    fn insert_texture(&mut self, file: &str, texture: Handle<Image>) {
        self.sprite_sheets.insert(file.to_string(), texture);
    }
}

impl ContentAssets for CompassContentAssets {
    fn prepared_appearances(&self) -> &PreparedAppearances {
        &self.prepared_appearances
    }
    fn sprite_sheet_data_set(&self) -> &Option<SpriteSheetDataSet> {
        &self.sheet_data_set
    }

    fn get_texture(&self, file: &str) -> Option<Handle<Image>> {
        Some(self.sprite_sheets.get(file)?.clone_weak())
    }

    fn get_atlas_layout(&self) -> Handle<TextureAtlasLayout> {
        self.atlas_layout_32_32.clone_weak()
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
