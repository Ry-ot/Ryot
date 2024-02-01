#![allow(dead_code)]
use crate::prelude::*;
use thiserror::Error;

const TILE_CONTENT_MAX_STACK_SIZE: usize = 10;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Tile content reached max stack size of {0}")]
    MaxStackSize(usize),
}

type Item = PreparedAppearance;

#[derive(Debug, Default)]
pub struct TileContent {
    top: Option<Item>,
    bottom: Option<Item>,
    ground: Option<Item>,
    content: Vec<Item>,
}

impl TileContent {
    pub fn add(&mut self, item: Item) -> Result<()> {
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
