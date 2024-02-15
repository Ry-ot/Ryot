use crate::item::{build_keys_for_area, ItemRepository};
use crate::{get_chunks_per_z, GetKey, Item, Tile};
use heed::types::Bytes;
use rayon::prelude::*;
use ryot::layer::Layer;
use ryot::lmdb;
use ryot::position::{Edges, TilePosition};
use ryot::prelude::{DatabaseName, SerdePostcard};
use std::collections::HashMap;

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
    fn get_for_area(&self, edges: &Edges) -> crate::Result<Vec<Tile>> {
        let chunks = get_chunks_per_z(edges);

        let result: Vec<Tile> = chunks
            .par_iter()
            .flat_map(|edges| {
                self.get_for_keys(build_keys_for_area(edges.min, edges.max))
                    .unwrap_or_else(|_| Vec::new())
            })
            .collect();

        Ok(result)
    }

    fn get_for_keys(&self, keys: Vec<Vec<u8>>) -> crate::Result<Vec<Tile>> {
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

    fn save_from_tiles(&self, tiles: Vec<Tile>) -> crate::Result<()> {
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
