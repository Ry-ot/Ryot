use bevy_ecs::prelude::Resource;
use bevy_sprite::TextureAtlasLayout;
use derive_more::{Deref, DerefMut};
use glam::{UVec2, Vec2};
use serde_repr::{Deserialize_repr, Serialize_repr};
use strum::EnumIter;

#[derive(Debug, Default, Clone, Resource, Deref, DerefMut)]
pub struct TextureAtlasLayouts(Vec<TextureAtlasLayout>);

#[derive(
    Serialize_repr, Deserialize_repr, Default, Eq, PartialEq, Debug, Copy, Clone, EnumIter, Hash,
)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::component::Component))]
#[repr(u32)]
pub enum SpriteLayout {
    #[default]
    OneByOne = 0,
    OneByTwo = 1,
    TwoByOne = 2,
    TwoByTwo = 3,
}

impl SpriteLayout {
    pub fn get_width(&self, tile_size: &UVec2) -> u32 {
        match self {
            SpriteLayout::OneByOne | SpriteLayout::OneByTwo => tile_size.x,
            SpriteLayout::TwoByOne | SpriteLayout::TwoByTwo => tile_size.x * 2,
        }
    }

    pub fn get_height(&self, tile_size: &UVec2) -> u32 {
        match self {
            SpriteLayout::OneByOne | SpriteLayout::TwoByOne => tile_size.y,
            SpriteLayout::OneByTwo | SpriteLayout::TwoByTwo => tile_size.y * 2,
        }
    }

    pub fn get_size(&self, tile_size: &UVec2) -> UVec2 {
        UVec2::new(self.get_width(tile_size), self.get_height(tile_size))
    }

    pub fn get_counts(&self, sheet_size: Vec2, tile_size: Vec2) -> Vec2 {
        let width = sheet_size.x / tile_size.x;
        let height = sheet_size.y / tile_size.y;
        match self {
            SpriteLayout::OneByOne => Vec2::new(width, height),
            SpriteLayout::OneByTwo => Vec2::new(width, height / 2.),
            SpriteLayout::TwoByOne => Vec2::new(width / 2., height),
            SpriteLayout::TwoByTwo => Vec2::new(width / 2., height / 2.),
        }
    }
}
