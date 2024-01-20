/*
 * Ryot - A free and open-source MMORPG server emulator
 * Copyright (©) 2023 Lucas Grossi <lucas.ggrossi@gmail.com>
 * Repository: https://github.com/lgrossi/Ryot
 * License: https://github.com/lgrossi/Ryot/blob/main/LICENSE
 * Contributors: https://github.com/lgrossi/Ryot/graphs/contributors
 * Website: https://github.com/lgrossi/Ryot
 */
use crate::{build_map, Position};
use bevy::log::{error, info};
use bevy::prelude::{ResMut, Resource};
use heed::Env;
use ryot::{compress, decompress, Zstd};
use time_test::time_test;

#[derive(Resource)]
pub struct LmdbEnv(pub Option<Env>);

impl Default for LmdbEnv {
    fn default() -> Self {
        Self(None)
    }
}

pub fn init_env(mut env: ResMut<LmdbEnv>) {
    info!("Setting up LMDB");
    env.0 = Some(lmdb::create_env(lmdb::get_storage_path()).unwrap());
}

pub fn read_area(initial_pos: &Position, final_pos: &Position, env: ResMut<LmdbEnv>) {
    match &env.0 {
        Some(env) => {
            time_test!("Reading");
            let item_repository =
                crate::item::items_from_heed_lmdb::ItemsFromHeedLmdb::new(env.clone());
            let area = item_repository
                .get_for_area(initial_pos, final_pos)
                .unwrap();
            println!("Count: {:?}", area.len());
        }
        None => {
            error!("No LMDB env");
        }
    }
}

pub fn lmdb_example() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let env = lmdb::create_env(lmdb::get_storage_path())?;
    let item_repository = crate::item::items_from_heed_lmdb::ItemsFromHeedLmdb::new(env.clone());
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
        let area = item_repository.get_for_area(&initial_pos, &final_pos)?;
        println!("Count: {}", area.len());
    }

    // env.prepare_for_closing().wait();
    // lmdb::compact()?;

    {
        time_test!("Compressing");
        compress::<Zstd>(
            lmdb::get_storage_path().join("data.mdb").to_str().unwrap(),
            Some(3),
        )?;
    }

    {
        time_test!("Decompressing");
        decompress::<Zstd>(
            lmdb::get_storage_path()
                .join("data.mdb.snp")
                .to_str()
                .unwrap(),
        )?;
    }

    Ok(())
}