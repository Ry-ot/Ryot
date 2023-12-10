use std::error::Error;

use heed::types::*;
use time_test::time_test;
use tokio::task;
use ryot::compass::{build_map, Item, KeyOption, Position};
use ryot::lmdb::{compress, DatabaseName, decompress, Lmdb, SerdePostcard, Zstd};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let lmdb = Lmdb::new()?;

    let (mut wtxn, db) = lmdb.init::<Bytes, SerdePostcard<Item>>(DatabaseName::Tiles)?;

    let map = {
        time_test!("Building compass");
        build_map()
    };

    println!("Tiles: {}", map.tiles.len());
    {
        time_test!("Writing");
        for tile in map.tiles {
            let item = tile.item.unwrap();

            db.put(
                &mut wtxn,
                &KeyOption::Position(tile.position.clone()).get_key_bytes(),
                &item
            )?;
        }
        wtxn.commit()?;
    }

    let initial_pos = Position::new(60000, 60000, 0);
    let final_pos = Position::new(61100, 61100, 15);

    let chunks = get_chunks_per_z(initial_pos, final_pos);
    println!("Chunks: {}", chunks.len());
    // exit(0);

    let mut handles = vec![];

    for (start, end) in chunks {
        let lmdb = lmdb.clone();
        handles.push(task::spawn_blocking(move || {
            // let mut tiles = vec![];
            let mut count = 0;
            let (rtxn, rodb) = lmdb.ro::<Bytes, SerdePostcard<Item>>(DatabaseName::Tiles).unwrap();
            let rodb = rodb.unwrap();
            for x in start.x..=end.x {
                for y in start.y..=end.y {
                    let key = KeyOption::Position(Position::new(x, y, start.z)).get_key_bytes();
                    let tile: Option<Item> = rodb.get(&rtxn, &key).unwrap();
                    if let Some(_) = tile {
                        // tiles.push(tile);
                        count += 1;
                    }
                }
            }
            rtxn.commit().expect("Failed to commit the transaction");
            count
        }));
    }

    // env.copy_to_file(Path::new("target").join("heed.mdb/bkp.mdb"), CompactionOption::Enabled).expect("Failed to copy the database");
    //
    // env.prepare_for_closing().expect("Failed to prepare for closing");
    //
    // std::fs::remove_file(Path::new("target").join("heed.mdb/data.mdb")).expect("Failed to remove the database");
    // env.copy_to_file(Path::new("target").join("heed.mdb/bkp.mdb"), CompactionOption::Enabled).expect("Failed to copy the database");

    {
        time_test!("Reading");
        let mut count = 0;
        for handle in handles {
            // count += handle.join().unwrap();
            count += handle.await?;
        }
        println!("Count: {}", count);
    }

    {
        time_test!("Compressing");
        compress::<Zstd>(
            Lmdb::get_storage_path().join("data.mdb").to_str().unwrap(),
            Some(3)
        )?;
    }

    {
        time_test!("Decompressing");
        decompress::<Zstd>(
            Lmdb::get_storage_path().join("data.mdb.snp").to_str().unwrap()
        )?;
    }

    Ok(())
}

fn get_chunks_per_z(initial_pos: Position, final_pos: Position) -> Vec<(Position, Position)> {
    let mut chunks = Vec::new();
    let n = 5;

    for z in initial_pos.z..=final_pos.z {
        for i in 1..=n {
            let y_divided_by_6 = (final_pos.y - initial_pos.y) / n;
            let chunk_start = Position::new(initial_pos.x, initial_pos.y + y_divided_by_6 * (i - 1), z);
            let chunk_end = Position::new(final_pos.x, initial_pos.y + y_divided_by_6 * i, z);
            chunks.push((chunk_start, chunk_end));
        }
    }

    chunks
}
