use bevy_asset::Handle;
use bevy_ecs::prelude::Resource;
use bevy_sprite::TextureAtlasLayout;
use derive_more::{Deref, DerefMut};

pub trait AtlasLayoutsAsset: crate::core::AssetResource {
    fn atlas_layouts(&self) -> &Vec<Handle<TextureAtlasLayout>>;
}

#[derive(Debug, Default, Clone, Resource, Deref, DerefMut)]
pub struct TextureAtlasLayouts(Vec<TextureAtlasLayout>);
