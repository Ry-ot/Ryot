use bevy_ecs::prelude::Resource;
use bevy_sprite::TextureAtlasLayout;
use derive_more::{Deref, DerefMut};

#[derive(Debug, Default, Clone, Resource, Deref, DerefMut)]
pub struct TextureAtlasLayouts(Vec<TextureAtlasLayout>);
