use bevy::prelude::{Camera, Changed, Commands, Local, Query, Res, ResMut, With};
use heed::types::Bytes;
use heed::Env;
use log::error;
use ryot::bevy_ryot::map::MapTiles;
use ryot::lmdb::{build_map, DatabaseName, Item, ItemRepository, ItemsFromHeedLmdb, SerdePostcard};
use ryot::position::{Sector, TilePosition};
use ryot::prelude::drawing::{DrawingBundle, LoadTileContent};
use ryot::prelude::lmdb::LmdbEnv;
use ryot::prelude::{compress, decompress, AppearanceDescriptor, Zstd};
use ryot::{lmdb, Layer};
use std::collections::HashMap;
use time_test::time_test;

pub fn init_tiles_db(lmdb_env: Res<LmdbEnv>) -> color_eyre::Result<()> {
    let env = lmdb_env.clone();
    let (wtxn, _) =
        lmdb::rw::<Bytes, SerdePostcard<HashMap<Layer, Item>>>(&env, DatabaseName::Tiles)?;

    wtxn.commit()?;

    Ok(())
}

pub fn read_area(
    tiles: Res<MapTiles>,
    env: ResMut<LmdbEnv>,
    mut commands: Commands,
    mut last_area: Local<Sector>,
    sector_query: Query<&Sector, (With<Camera>, Changed<Sector>)>,
) {
    let Ok(sector) = sector_query.get_single() else {
        return;
    };

    let sector = *sector * 1.5;

    for area in *last_area - sector {
        load_area(area, env.clone(), &mut commands, &tiles);
    }

    *last_area = sector;
}

pub fn load_area(sector: Sector, env: Env, commands: &mut Commands, tiles: &Res<MapTiles>) {
    let item_repository = ItemsFromHeedLmdb::new(env);

    match item_repository.get_for_area(&sector) {
        Ok(area) => {
            let mut bundles = vec![];

            for tile in area {
                for (layer, item) in tile.items {
                    if let Some(tile) = tiles.get(&tile.position) {
                        if tile.peek_for_layer(layer).is_some() {
                            continue;
                        }
                    }

                    bundles.push(DrawingBundle::new(
                        layer,
                        tile.position,
                        AppearanceDescriptor::object(item.id as u32),
                    ));
                }
            }

            commands.add(LoadTileContent::from_bundles(bundles));
        }
        Err(e) => {
            error!("Failed to read area: {}", e);
        }
    }
}

pub fn lmdb_example() -> Result<(), Box<dyn std::error::Error>> {
    let env = lmdb::create_env(lmdb::get_storage_path())?;
    let item_repository = ItemsFromHeedLmdb::new(env.clone());
    let z_size = 1;

    let map = {
        time_test!("Building ryot_compass");
        build_map(z_size)
    };

    {
        time_test!("Writing");
        item_repository.save_from_tiles(map.tiles)?;
    }

    let initial_pos = TilePosition::new(-550, -550, 0);
    let final_pos = TilePosition::new(550, 550, z_size - 1);

    {
        time_test!("Reading");
        let area = item_repository.get_for_area(&Sector::new(initial_pos, final_pos))?;
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
