use bevy_asset::Handle;
use bevy_sprite::TextureAtlasLayout;

pub trait AtlasLayoutsAsset: crate::RyotAsset {
    fn atlas_layouts(&self) -> &Vec<Handle<TextureAtlasLayout>>;
}
