use std::error::Error;

use heed::types::*;
use rand::Rng;
use time_test::time_test;
use tokio::task;
use ryot::{compress, decompress, lmdb, Zstd};
use ryot::lmdb::{DatabaseName, SerdePostcard};
use ryot_compass::{build_map, get_chunks_per_z, GetKey, Item, Position};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let env = lmdb::create_env(lmdb::get_storage_path())?;
    let (mut wtxn, db) = lmdb::rw::<Bytes, SerdePostcard<Item>>(&env, DatabaseName::Tiles)?;

    let map = {
        time_test!("Building ryot_compass");
        build_map()
    };

    println!("Tiles: {}", map.tiles.len());
    {
        time_test!("Writing");
        for tile in map.tiles {
            let item = tile.item.unwrap();
            let key = tile.position.get_binary_key();

            db.delete(&mut wtxn, &key)?;
            db.put(&mut wtxn, &key, &item)?;
        }
        wtxn.commit()?;
    }

    let initial_pos = Position::new(60000, 60000, 0);
    let final_pos = Position::new(61100, 61100, 15);

    let chunks = get_chunks_per_z(initial_pos, final_pos);
    println!("Chunks: {}", chunks.len());

    let mut handles = vec![];

    for (start, end) in chunks {
        let env = env.clone();
        handles.push(task::spawn_blocking(move || {
            // let mut tiles = vec![];
            let mut count = 0;

            let (rtxn, rodb) = lmdb::ro::<Bytes, SerdePostcard<Item>>(&env, DatabaseName::Tiles).unwrap();
            for x in start.x..=end.x {
                for y in start.y..=end.y {
                    let key = Position::new(x, y, start.z).get_binary_key();
                    let tile: Option<Item> = rodb.get(&rtxn, &key).unwrap();
                    if let Some(tile) = tile {
                        if rand::thread_rng().gen_range(0..=5_000_000) == 0 {
                            println!("Tile: {:?}", tile);
                        }
                        // tiles.push(tile);
                        count += 1;
                    }
                }
            }
            rtxn.commit().expect("Failed to commit the transaction");
            count
        }));
    }

    {
        time_test!("Reading");
        let mut count = 0;
        for handle in handles {
            // count += handle.join().unwrap();
            count += handle.await?;
        }
        println!("Count: {}", count);
    }

    // env.prepare_for_closing().wait();
    // lmdb::compact()?;

    {
        time_test!("Compressing");
        compress::<Zstd>(
            lmdb::get_storage_path().join("data.mdb").to_str().unwrap(),
            Some(3)
        )?;
    }

    {
        time_test!("Decompressing");
        decompress::<Zstd>(
            lmdb::get_storage_path().join("data.mdb.snp").to_str().unwrap()
        )?;
    }

    Ok(())
}


