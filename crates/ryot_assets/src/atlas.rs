use bevy_asset::Handle;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_ecs::prelude::Resource;
use bevy_sprite::TextureAtlasLayout;

pub trait AtlasLayoutsAsset: Resource + AssetCollection + Send + Sync + 'static {
    fn atlas_layouts(&self) -> &Vec<Handle<TextureAtlasLayout>>;
}
