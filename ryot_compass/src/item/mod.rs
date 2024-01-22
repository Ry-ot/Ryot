use crate::{GetKey, Item, Position, Tile};

#[cfg(all(feature = "lmdb", not(target_arch = "wasm32")))]
mod items_from_heed_lmdb;

#[cfg(all(feature = "lmdb", not(target_arch = "wasm32")))]
pub use items_from_heed_lmdb::ItemsFromHeedLmdb;

pub trait ItemRepository {
    fn get_for_area(
        &self,
        initial_pos: &Position,
        final_pos: &Position,
    ) -> crate::Result<Vec<(Vec<u8>, Item)>>;
    fn get_for_keys(&self, keys: Vec<Vec<u8>>) -> crate::Result<Vec<(Vec<u8>, Item)>>;
    fn save_from_tiles(&self, items: Vec<Tile>) -> crate::Result<()>;
}

pub fn build_keys_for_area(initial_pos: Position, final_pos: Position) -> Vec<Vec<u8>> {
    let mut keys = vec![];

    for x in initial_pos.x..=final_pos.x {
        for y in initial_pos.y..=final_pos.y {
            for z in initial_pos.z..=final_pos.z {
                keys.push(Position::new(x, y, z).get_binary_key());
            }
        }
    }

    keys
}
