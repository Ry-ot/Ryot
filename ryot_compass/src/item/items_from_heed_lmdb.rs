/*
 * Ryot - A free and open-source MMORPG server emulator
 * Copyright (©) 2023 Lucas Grossi <lucas.ggrossi@gmail.com>
 * Repository: https://github.com/lgrossi/Ryot
 * License: https://github.com/lgrossi/Ryot/blob/main/LICENSE
 * Contributors: https://github.com/lgrossi/Ryot/graphs/contributors
 * Website: https://github.com/lgrossi/Ryot
 */
use heed::types::Bytes;
use log::debug;
use rayon::prelude::*;
use ryot::lmdb;
use ryot::lmdb::{DatabaseName, SerdePostcard};
use crate::{get_chunks_per_z, GetKey, Item, Position, Tile};
use crate::item::{build_keys_for_area, ItemRepository};

#[derive(Clone)]
pub struct ItemsFromHeedLmdb {
    env: heed::Env,
}

impl ItemsFromHeedLmdb {
    pub fn new(env: heed::Env) -> Self {
        Self { env }
    }
}

impl ItemRepository for ItemsFromHeedLmdb {
    fn get_for_area(&self, initial_pos: &Position, final_pos: &Position) -> crate::Result<Vec<(Vec<u8>, Item)>> {
        let chunks = get_chunks_per_z(initial_pos, final_pos);
        debug!("Chunks: {}", chunks.len());

        let result: Vec<_> = chunks.par_iter()
            .flat_map(|(start, end)| {
                self.get_for_keys(build_keys_for_area(*start, *end)).unwrap_or_else(|_| Vec::new())
            })
            .collect();

        Ok(result)
    }

    fn get_for_keys(&self, keys: Vec<Vec<u8>>) -> crate::Result<Vec<(Vec<u8>, Item)>> {
        let mut tiles = vec![];

        let (rtxn, rodb) = lmdb::ro::<Bytes, SerdePostcard<Item>>(
            &self.env, DatabaseName::Tiles
        )?;

        for key in keys {
            let tile: Option<Item> = rodb.get(&rtxn, &key)?;
            if let Some(tile) = tile {
                tiles.push((key.clone(), tile));
            }
        }

        rtxn.commit()?;

        Ok(tiles)
    }

    fn save_from_tiles(&self, tiles: Vec<Tile>) -> crate::Result<()> {
        let (mut wtxn, db) = lmdb::rw::<Bytes, SerdePostcard<Item>>(&self.env, DatabaseName::Tiles)?;

        for tile in tiles {
            let item = tile.item.unwrap();
            let key = tile.position.get_binary_key();

            db.delete(&mut wtxn, &key)?;
            db.put(&mut wtxn, &key, &item)?;
        }

        wtxn.commit()?;

        Ok(())
    }
}