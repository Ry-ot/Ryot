use std::error::Error;

use heed::types::*;
use heed::{Database};
use time_test::time_test;
use ryot::compass::{build_map, Item, KeyOption, Position};
use ryot::lmdb::{compress, DatabaseName, decompress, Lmdb, SerdePostcard, Zstd};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let db = Lmdb::new()?;

    db.init::<Bytes, SerdePostcard<Item>>(DatabaseName::Tiles)?;
    let (mut wtxn, db) = db.init::<Bytes, SerdePostcard<Item>>(DatabaseName::Tiles)?;

    let tile = db.get(&wtxn, &KeyOption::Position(Position{x: 60300, y: 60300, z: 0}).get_key_bytes()).unwrap();
    println!("serde-json:\t{:?}", tile);
    // exit(0);

    let map = {
        time_test!("Building compass");
        build_map()
    };

    println!("Tiles: {}", map.tiles.len());

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
    // iwtxn.commit()?;

    // // let rtxn = env.read_txn().expect("Failed to create read transaction");
    // // let rodb = env.open_database::<Str, SerdeBincode<Tile>>(&rtxn, Some("serde-json"))?.unwrap();
    // // let tile: Option<Tile> = rodb.get(&rtxn, &*format!("hello-{}", 1)).unwrap();
    // // println!("serde-json:\t{:?}", tile);
    //
    // let n = 10_000_000;
    //
    // // println!("Total: {}", db.len(&wtxn)?);
    // // {
    // //     time_test!("Writing");
    // //     for i in 0..n {
    // //         db.put_with_flags(&mut wtxn, PutFlags::APPEND_DUP, &*format!("hello-{}", i), &hello)?;
    // //         if i % 100_000 == 0 {
    // //             println!("Writed {}%", (100 * i) / n);
    // //         }
    // //     }
    // //
    // //     wtxn.commit()?;
    // // }
    //
    //
    // // env.copy_to_file(Path::new("target").join("heed.mdb/bkp.mdb"), CompactionOption::Enabled).expect("Failed to copy the database");
    // //
    // // env.prepare_for_closing().expect("Failed to prepare for closing");
    // //
    // // std::fs::remove_file(Path::new("target").join("heed.mdb/data.mdb")).expect("Failed to remove the database");
    // // env.copy_to_file(Path::new("target").join("heed.mdb/bkp.mdb"), CompactionOption::Enabled).expect("Failed to copy the database");
    //
    // let vec = (0..n).collect::<Vec<_>>();
    // let mut handles = vec![];
    // let chunks = vec.chunks(n/100);
    //
    // println!("Starting reading");
    //
    // for chunk in chunks {
    //     let chunk = chunk.to_vec();
    //     let env = env.clone();
    //     handles.push(task::spawn_blocking(move || {
    //         let mut tiles = vec![];
    //         let mut size = 0;
    //         let rtxn = env.read_txn().expect("Failed to create read transaction");
    //         let rodb = env.open_database::<Str, SerdeBincode<Tile>>(&rtxn, Some("serde-json")).unwrap().unwrap();
    //         chunk.iter().for_each(|i|{
    //             let tile: Option<Tile> = rodb.get(&rtxn, &*format!("hello-{}", i)).unwrap();
    //             tiles.push(tile);
    //             // println!("serde-json:\t{:?}", tile);
    //             size += 1;
    //         });
    //         rtxn.commit().expect("Failed to commit the transaction");
    //         sleep(Duration::from_secs(5));
    //         size
    //     }));
    // };
    //
    // {
    //     time_test!("Reading");
    //     for handle in handles {
    //         println!("Joining on task {:?}", handle.await?);
    //     }
    // }
    //
    // env.prepare_for_closing();
    //

    {
        time_test!("DECompressing");
        compress::<Zstd>(
            Lmdb::get_storage_path().join("data.mdb").to_str().unwrap(),
            Some(3)
        )?;

        decompress::<Zstd>(
            Lmdb::get_storage_path().join("data.mdb.snp").to_str().unwrap()
        )?;
    }

    Ok(())
}