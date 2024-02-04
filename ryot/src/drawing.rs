#![allow(dead_code)]

use crate::appearances;
use crate::prelude::*;
use bevy::prelude::Component;
use bevy::utils::HashMap;
use thiserror::Error;

const TILE_CONTENT_MAX_STACK_SIZE: usize = 10;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Tile content reached max stack size of {0}")]
    MaxStackSize(usize),
}

#[derive(Debug, Default)]
pub struct TileContent {
    pub layers: HashMap<Layer, PreparedAppearance>,
    pub content: Vec<PreparedAppearance>,
}

impl TileContent {
    pub fn add(&mut self, item: PreparedAppearance) -> Result<()> {
        match item.layer {
            Layer::None => {
                if self.content.len() >= TILE_CONTENT_MAX_STACK_SIZE {
                    return Err(Error::MaxStackSize(TILE_CONTENT_MAX_STACK_SIZE));
                }

                self.content.push(item);
            }
            layer => {
                self.layers.insert(layer, item);
            }
        }

        Ok(())
    }

    pub fn get_sprites_to_be_drawn(&self) -> Vec<u32> {
        let mut sprites = vec![];

        for layer in &[Layer::Bottom, Layer::Ground] {
            if let Some(item) = self.layers.get(layer) {
                sprites.push(item.main_sprite_id);
            }
        }

        // if content has repeated sprites one after the other, skip them
        let mut last_sprite_id = None;
        for item in &self.content {
            if let Some(last_sprite_id) = last_sprite_id {
                if last_sprite_id == item.main_sprite_id {
                    continue;
                }
            }

            sprites.push(item.main_sprite_id);
            last_sprite_id = Some(item.main_sprite_id);
        }

        for layer in &[Layer::Top] {
            if let Some(item) = self.layers.get(layer) {
                sprites.insert(0, item.main_sprite_id);
            }
        }

        sprites
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone, Default, Component)]
pub enum Layer {
    #[default]
    None,
    Top,
    Bottom,
    Ground,
}

impl Layer {
    pub fn base_z_offset(&self) -> i32 {
        match self {
            Self::Top => 7,
            Self::Bottom => 3,
            Self::Ground => 0,
            Self::None => 5,
        }
    }
}

impl From<Option<appearances::AppearanceFlags>> for Layer {
    fn from(flags: Option<appearances::AppearanceFlags>) -> Self {
        match flags {
            Some(flags) if flags.top.is_some() => Self::Top,
            Some(flags) if flags.bottom.is_some() => Self::Bottom,
            Some(flags) if flags.bank.is_some() || flags.clip.is_some() => Self::Ground,
            _ => Self::None,
        }
    }
}
