/*
 * Ryot - A free and open-source MMORPG server emulator
 * Copyright (©) 2023 Lucas Grossi <lucas.ggrossi@gmail.com>
 * Repository: https://github.com/lgrossi/Ryot
 * License: https://github.com/lgrossi/Ryot/blob/main/LICENSE
 * Contributors: https://github.com/lgrossi/Ryot/graphs/contributors
 * Website: https://github.com/lgrossi/Ryot
 */

use std::{result, thread};
use std::fmt::{Display, Formatter};
use std::sync::Arc;
use heed::types::Bytes;
use log::debug;
use ryot::lmdb;
use ryot::lmdb::{DatabaseName, SerdePostcard};
use crate::{get_chunks_per_z, GetKey, Item, Position, Tile};

#[derive(Debug)]
pub enum Error{
    DatabaseError(String),
}

impl From<heed::Error> for Error {
    fn from(e: heed::Error) -> Self {
        Error::DatabaseError(e.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DatabaseError(e) => write!(f, "Database error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = result::Result<T, Error>;

pub trait ItemRepository {
    fn get_for_area(&self, initial_pos: Position, final_pos: Position) -> Result<Vec<(Vec<u8>, Item)>>;
    fn get_for_keys(&self, keys: Vec<Vec<u8>>) -> Result<Vec<(Vec<u8>, Item)>>;
    fn save_from_tiles(&self, items: Vec<Tile>) -> Result<()>;
}

#[derive(Clone)]
pub struct ItemsFromHeedLmdb {
    env: Arc<heed::Env>,
}

impl ItemsFromHeedLmdb {
    pub fn new(env: Arc<heed::Env>) -> Self {
        Self { env }
    }

    fn read_from_lmdb(&self, keys: Vec<Vec<u8>>) -> thread::JoinHandle<Result<Vec<(Vec<u8>, Item)>>> {
        let repo = self.clone();
        thread::spawn(move || { repo.get_for_keys(keys) })
    }
}

impl ItemRepository for ItemsFromHeedLmdb {
    fn get_for_area(&self, initial_pos: Position, final_pos: Position) -> Result<Vec<(Vec<u8>, Item)>> {
        let mut handles = vec![];

        let chunks = get_chunks_per_z(initial_pos, final_pos);
        debug!("Chunks: {}", chunks.len());

        for (start, end) in chunks {
            handles.push(self.read_from_lmdb(build_keys_for_area(start, end)));
        }

        let mut result = Vec::new();

        for handle in handles {
            result.extend(handle.join().unwrap().unwrap());
        }

        Ok(result)
    }

    fn get_for_keys(&self, keys: Vec<Vec<u8>>) -> Result<Vec<(Vec<u8>, Item)>> {
        let mut tiles = vec![];

        let (rtxn, rodb) = lmdb::ro::<Bytes, SerdePostcard<Item>>(&self.env, DatabaseName::Tiles)?;

        for key in keys {
            let tile: Option<Item> = rodb.get(&rtxn, &key)?;
            if let Some(tile) = tile {
                tiles.push((key.clone(), tile));
            }
        }

        rtxn.commit()?;

        Ok(tiles)
    }

    fn save_from_tiles(&self, tiles: Vec<Tile>) -> Result<()> {
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

pub fn build_keys_for_area(
    initial_pos: Position,
    final_pos: Position,
) -> Vec<Vec<u8>> {
    let mut keys = vec![];

    for x in initial_pos.x..=final_pos.x {
        for y in initial_pos.y..=final_pos.y {
            let key = Position::new(x, y, initial_pos.z).get_binary_key();
            keys.push(key);
        }
    }

    keys
}