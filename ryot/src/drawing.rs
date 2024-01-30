#![allow(dead_code)]
use crate::appearances::*;
use glam::UVec3;
use thiserror::Error;

const TILE_CONTENT_MAX_STACK_SIZE: usize = 10;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Missing item")]
    MissingItem,
    #[error("Tile content reached max stack size of {0}")]
    MaxStackSize(usize),
}

pub enum ToBeAdded {
    Item(Item),
    Appearance(Appearance),
}

type Item = TileItem;

pub struct TileItem {
    id: u32,
    name: String,
    main_sprite_id: u32,
    frame_groups: Vec<FrameGroup>,
    flags: Option<AppearanceFlags>,
}

impl TileItem {
    pub fn from_appearance(item: Appearance) -> Option<TileItem> {
        let id = item.id?;
        let main_frame = item.frame_group.clone().first()?.clone();
        let main_sprite_id = *main_frame.sprite_info?.sprite_id.first()?;

        Some(TileItem {
            id: item.id?,
            name: item.name.unwrap_or(id.to_string()),
            main_sprite_id,
            frame_groups: item.frame_group.clone(),
            flags: item.flags.clone(),
        })
    }
}

pub struct Tile {
    position: UVec3,
    top: Option<Item>,
    bottom: Option<Item>,
    ground: Option<Item>,
    content: Vec<Item>,
}

impl Tile {
    pub fn add(&mut self, to_be_added: ToBeAdded) -> Result<()> {
        let item: Option<Item> = match to_be_added {
            ToBeAdded::Item(item) => Some(item),
            ToBeAdded::Appearance(appearance) => TileItem::from_appearance(appearance),
        };

        let Some(item) = item else {
            return Err(Error::MissingItem);
        };

        match item.flags.clone() {
            Some(flags) => {
                if flags.top.is_some() {
                    self.top = Some(item);
                } else if flags.bottom.is_some() {
                    self.bottom = Some(item);
                } else if flags.bank.is_some() || flags.clip.is_some() {
                    self.ground = Some(item);
                }
            }
            None => {
                if self.content.len() >= TILE_CONTENT_MAX_STACK_SIZE {
                    return Err(Error::MaxStackSize(TILE_CONTENT_MAX_STACK_SIZE));
                }

                self.content.push(item);
            }
        }

        Ok(())
    }
}
