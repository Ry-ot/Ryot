use crate::layer::Layer;
use crate::lmdb;
use crate::lmdb::*;
use crate::position::{Sector, TilePosition};
use heed::types::Bytes;
use rayon::prelude::*;
use std::collections::HashMap;

pub fn build_keys_for_area(initial_pos: TilePosition, final_pos: TilePosition) -> Vec<Vec<u8>> {
    let mut keys = vec![];

    for x in initial_pos.x..=final_pos.x {
        for y in initial_pos.y..=final_pos.y {
            for z in initial_pos.z..=final_pos.z {
                keys.push(TilePosition::new(x, y, z).get_binary_key());
            }
        }
    }

    keys
}

pub trait ItemRepository {
    fn get_for_area(&self, sector: &Sector) -> error::Result<Vec<Tile>>;
    fn get_for_keys(&self, keys: Vec<Vec<u8>>) -> error::Result<Vec<Tile>>;
    fn save_from_tiles(&self, items: Vec<Tile>) -> error::Result<()>;
}

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
    fn get_for_area(&self, sector: &Sector) -> error::Result<Vec<Tile>> {
        let chunks = get_chunks_per_z(sector);

        let result: Vec<Tile> = chunks
            .par_iter()
            .flat_map(|sector| {
                self.get_for_keys(build_keys_for_area(sector.min, sector.max))
                    .unwrap_or_else(|_| Vec::new())
            })
            .collect();

        Ok(result)
    }

    fn get_for_keys(&self, keys: Vec<Vec<u8>>) -> error::Result<Vec<Tile>> {
        let mut tiles = vec![];

        let (rtxn, rodb) =
            lmdb::ro::<Bytes, SerdePostcard<HashMap<Layer, Item>>>(&self.env, DatabaseName::Tiles)?;

        for key in keys {
            let tile: Option<HashMap<Layer, Item>> = rodb.get(&rtxn, &key)?;
            if let Some(tile) = tile {
                tiles.push(Tile::new(TilePosition::from_binary_key(&key), tile));
            }
        }

        rtxn.commit()?;

        Ok(tiles)
    }

    fn save_from_tiles(&self, tiles: Vec<Tile>) -> error::Result<()> {
        let (mut wtxn, db) =
            lmdb::rw::<Bytes, SerdePostcard<HashMap<Layer, Item>>>(&self.env, DatabaseName::Tiles)?;

        for tile in tiles {
            let item = tile.items;
            let key = tile.position.get_binary_key();

            db.delete(&mut wtxn, &key)?;
            db.put(&mut wtxn, &key, &item)?;
        }

        wtxn.commit()?;

        Ok(())
    }
}
