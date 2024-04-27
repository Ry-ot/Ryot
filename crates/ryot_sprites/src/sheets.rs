use bevy_ecs::prelude::Resource;
use derive_more::{Deref, DerefMut};
use ryot_core::prelude::SpriteSheet;

/// This is a collection of sprite sheets.
/// It contains the sprite sheets and the sprite sheet config.
/// The sprite sheet config is used to calculate the position and size of a sprite in the sprite
/// sheet.
#[derive(Debug, Default, Clone, Deref, DerefMut, Resource)]
pub struct SpriteSheets(Vec<SpriteSheet>);

impl<T> From<&[T]> for SpriteSheets
where
    T: Into<Option<SpriteSheet>> + Clone,
{
    fn from(content: &[T]) -> Self {
        let sprite_sheets = content
            .iter()
            .filter_map(|content_type| content_type.clone().into())
            .collect::<Vec<_>>();

        Self(sprite_sheets)
    }
}

impl<T> From<Vec<T>> for SpriteSheets
where
    T: Into<Option<SpriteSheet>> + Clone,
{
    fn from(content: Vec<T>) -> Self {
        content.as_slice().into()
    }
}

impl SpriteSheets {
    /// Returns the sprite sheet that contains the given sprite id.
    /// Returns None if the sprite id is not in any of the sprite sheets.
    pub fn get_by_sprite_id(&self, sprite_id: u32) -> Option<&SpriteSheet> {
        self.iter().find(|sheet| sheet.has_sprite(sprite_id))
    }
}
