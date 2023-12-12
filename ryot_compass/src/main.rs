use std::error::Error;

use time_test::time_test;
use ryot::{compress, decompress, lmdb, Zstd};
use ryot_compass::{build_map, Position};
use ryot_compass::item::{ItemRepository, ItemsFromHeedLmdb};

fn main() -> Result<(), Box<dyn Error>> {
    let env = lmdb::create_env(lmdb::get_storage_path())?;
    let item_repository = ItemsFromHeedLmdb::new(env.clone());
    let z_size = 15;

    let map = {
        time_test!("Building ryot_compass");
        build_map(z_size)
    };

    {
        time_test!("Writing");
        item_repository.save_from_tiles(map.tiles)?;
    }

    let initial_pos = Position::new(60000, 60000, 0);
    let final_pos = Position::new(61000, 61000, z_size - 1);

    {
        time_test!("Reading");
        let area = item_repository.get_for_area(initial_pos, final_pos)?;
        println!("Count: {}", area.len());
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

