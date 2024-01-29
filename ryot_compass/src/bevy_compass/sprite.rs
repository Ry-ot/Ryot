use bevy::asset::Handle;
use bevy::prelude::{Image, Resource};
use bevy::utils::HashMap;
use bevy_asset_loader::asset_collection::AssetCollection;
use ryot::bevy_ryot::sprites::SpriteAssets;

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
