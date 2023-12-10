use std::error::Error;
use std::{fs};
use std::path::Path;
use std::process::exit;
use std::io::{Write, Read, BufReader, BufWriter};
use std::thread::sleep;
use std::time::Duration;
// use async_std::task::sleep;
use futures::StreamExt;

use heed::types::*;
use heed::{CompactionOption, Database, EnvOpenOptions, PutFlags};
use serde::{Deserialize, Serialize};
use time_test::time_test;
use tokio::task;

#[derive(Debug, Serialize, Deserialize)]
struct Hello<'a> {
    string: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
struct Tile<'a> {
    string: &'a str,
    string2: &'a str,
    string3: &'a str,
    string4: &'a str,
    string5: &'a str,
    string6: &'a str,
    string7: &'a str,
    hello: Hello<'a>,
}

fn compress<R: Read, W: Write>(
    mut src: R,
    mut dst: W,
) -> anyhow::Result<()> {
    // Read the entire src into memory and compress it.
    let mut buf = Vec::with_capacity(10 * (1 << 20));
    src.read_to_end(&mut buf)?;
    let compressed = snap::raw::Encoder::new().compress_vec(&buf)?;
    dst.write_all(&compressed)?;
    Ok(())
}

fn decompress<R: Read, W: Write>(
    mut src: R,
    mut dst: W,
) -> anyhow::Result<()> {
    // Read the entire src into memory and decompress it.
    let mut buf = Vec::with_capacity(10 * (1 << 20));
    src.read_to_end(&mut buf)?;
    let decompressed =
        snap::raw::Decoder::new().decompress_vec(&buf)?;
    dst.write_all(&decompressed)?;
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("target").join("heed.mdb/data.mdb");

    fs::create_dir_all(&path)?;
    let env = EnvOpenOptions::new()
        .map_size(10 * 1024 * 1024 * 1024) // 10GB
        .max_dbs(20)
        .open(path.clone())?;

    let mut wtxn = env.write_txn()?;
    let db: Database<Str, SerdeBincode<Tile>> = env.create_database(&mut wtxn, Some("serde-json"))?;
    wtxn.commit()?;

    // let rtxn = env.read_txn().expect("Failed to create read transaction");
    // let rodb = env.open_database::<Str, SerdeBincode<Tile>>(&rtxn, Some("serde-json"))?.unwrap();
    // let tile: Option<Tile> = rodb.get(&rtxn, &*format!("hello-{}", 1)).unwrap();
    // println!("serde-json:\t{:?}", tile);

    let hello = Tile {
        string: "hello",
        string2: "hello",
        string3: "hello",
        string4: "hello",
        string5: "hello",
        string6: "hello",
        string7: "hello",
        hello: Hello { string: "hellw" },
    };

    let n = 10_000_000;

    // println!("Total: {}", db.len(&wtxn)?);
    // {
    //     time_test!("Writing");
    //     for i in 0..n {
    //         db.put_with_flags(&mut wtxn, PutFlags::APPEND_DUP, &*format!("hello-{}", i), &hello)?;
    //         if i % 100_000 == 0 {
    //             println!("Writed {}%", (100 * i) / n);
    //         }
    //     }
    //
    //     wtxn.commit()?;
    // }


    // env.copy_to_file(Path::new("target").join("heed.mdb/bkp.mdb"), CompactionOption::Enabled).expect("Failed to copy the database");
    //
    // env.prepare_for_closing().expect("Failed to prepare for closing");
    //
    // std::fs::remove_file(Path::new("target").join("heed.mdb/data.mdb")).expect("Failed to remove the database");
    // env.copy_to_file(Path::new("target").join("heed.mdb/bkp.mdb"), CompactionOption::Enabled).expect("Failed to copy the database");

    let vec = (0..n).collect::<Vec<_>>();
    let mut handles = vec![];
    let chunks = vec.chunks(n/100);

    println!("Starting reading");

    for chunk in chunks {
        let chunk = chunk.to_vec();
        let env = env.clone();
        handles.push(task::spawn_blocking(move || {
            let mut tiles = vec![];
            let mut size = 0;
            let rtxn = env.read_txn().expect("Failed to create read transaction");
            let rodb = env.open_database::<Str, SerdeBincode<Tile>>(&rtxn, Some("serde-json")).unwrap().unwrap();
            chunk.iter().for_each(|i|{
                let tile: Option<Tile> = rodb.get(&rtxn, &*format!("hello-{}", i)).unwrap();
                tiles.push(tile);
                // println!("serde-json:\t{:?}", tile);
                size += 1;
            });
            rtxn.commit().expect("Failed to commit the transaction");
            sleep(Duration::from_secs(5));
            size
        }));
    };

    {
        time_test!("Reading");
        for handle in handles {
            println!("Joining on task {:?}", handle.await?);
        }
    }

    env.prepare_for_closing();

    // let path = Path::new("target").join("heed.mdb/data.mdb/data.mdb");
    // let new_path = Path::new("target").join("heed.mdb/data.mdb.snp");
    //
    // let old_file = BufReader::new(fs::File::open(path)?);
    // let new_file = BufWriter::new(fs::File::create(&new_path)?);
    // {
    //     time_test!("Compressing");
    //     compress(old_file, new_file)?;
    // }
    //
    // let path = Path::new("target").join("heed.mdb/data.mdb.snp");
    // let new_path = Path::new("target").join("heed.mdb/data.mdb/data2.mdb");
    //
    // let old_file = BufReader::new(fs::File::open(path)?);
    // let new_file = BufWriter::new(fs::File::create(&new_path)?);
    // {
    //     time_test!("Descompressing");
    //     decompress(old_file, new_file)?;
    // }

    Ok(())
}