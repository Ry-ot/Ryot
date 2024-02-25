use crate::{ExportMap, LoadMap};
use bevy::prelude::{
    info, Camera, Changed, Commands, Entity, EventReader, Local, Query, Res, ResMut, Transform,
    With,
};
use heed::types::Bytes;
use heed::Env;
use log::{debug, error, warn};
use ryot::bevy_ryot::lmdb::LmdbCompactor;
use ryot::bevy_ryot::map::MapTiles;
use ryot::helpers::execute_async;
use ryot::lmdb::{
    build_map, get_storage_path, DatabaseName, Item, ItemRepository, ItemsFromHeedLmdb,
    SerdePostcard, MDB_FILE_NAME,
};
use ryot::position::{Sector, TilePosition};
use ryot::prelude::drawing::{DrawingBundle, LoadTileContent, TileComponent};
use ryot::prelude::lmdb::LmdbEnv;
use ryot::prelude::{compress, decompress, AppearanceDescriptor, Zstd};
use ryot::{lmdb, Layer};
use std::collections::HashMap;
use std::fs;
use std::sync::atomic::Ordering;
use time_test::time_test;

pub fn init_tiles_db(lmdb_env: Res<LmdbEnv>) -> color_eyre::Result<()> {
    let Some(env) = &lmdb_env.0 else {
        return Ok(());
    };

    let (wtxn, _) =
        lmdb::rw::<Bytes, SerdePostcard<HashMap<Layer, Item>>>(env, DatabaseName::Tiles)?;

    wtxn.commit()?;

    Ok(())
}

pub fn read_area(
    tiles: Res<MapTiles>,
    env: ResMut<LmdbEnv>,
    mut commands: Commands,
    mut last_area: Local<Sector>,
    mut load_map_events: EventReader<LoadMap>,
    sector_query: Query<&Sector, (With<Camera>, Changed<Sector>)>,
) {
    let Some(env) = &env.0 else {
        return;
    };

    let Ok(sector) = sector_query.get_single() else {
        return;
    };

    if load_map_events.read().len() > 0 {
        *last_area = Sector::default();
    }

    let sector = *sector * 1.5;

    for area in *last_area - sector {
        load_area(area, env.clone(), &mut commands, &tiles);
    }

    *last_area = sector;
}

pub fn reload_visible_area(
    tiles: Res<MapTiles>,
    env: ResMut<LmdbEnv>,
    mut commands: Commands,
    sector_query: Query<&Sector, With<Camera>>,
) {
    let Some(env) = &env.0 else {
        return;
    };

    for sector in sector_query.iter() {
        load_area(*sector, env.clone(), &mut commands, &tiles);
    }
}

pub fn load_map(
    mut env: ResMut<LmdbEnv>,
    mut commands: Commands,
    mut tiles: ResMut<MapTiles>,
    mut load_map_events: EventReader<LoadMap>,
    mut q_all_tiles: Query<Entity, With<TileComponent>>,
) -> color_eyre::Result<()> {
    if let Some(env) = &env.0 {
        env.clone().prepare_for_closing();
    }

    env.0 = None;

    for id in q_all_tiles.iter_mut() {
        commands.entity(id).despawn();
    }

    tiles.clear();

    fs::remove_file(get_storage_path().join(MDB_FILE_NAME)).ok();

    for LoadMap(path) in load_map_events.read() {
        match fs::copy(path.clone(), get_storage_path().join(MDB_FILE_NAME)) {
            Ok(bytes_copied) => info!("Map loaded: {} bytes", bytes_copied),
            Err(e) => {
                warn!("Failed to load map: {}", e);
                continue;
            }
        }
    }

    Ok(())
}

pub fn init_new_map(
    mut env: ResMut<LmdbEnv>,
    mut q_camera_transform: Query<&mut Transform, With<Camera>>,
) -> color_eyre::Result<()> {
    let new_env = lmdb::create_env(lmdb::get_storage_path()).expect("Failed to create LMDB env");
    let (wtxn, _) =
        lmdb::rw::<Bytes, SerdePostcard<HashMap<Layer, Item>>>(&new_env, DatabaseName::Tiles)?;
    wtxn.commit()?;

    *q_camera_transform.single_mut() = Transform::IDENTITY;

    env.0 = Some(new_env);

    Ok(())
}

pub fn export_map(
    env: Res<LmdbEnv>,
    lmdb_compactor: ResMut<LmdbCompactor>,
    mut map_export_events: EventReader<ExportMap>,
) -> color_eyre::Result<()> {
    let Some(env) = &env.0 else {
        return Ok(());
    };

    for ExportMap(destination) in map_export_events.read() {
        if !lmdb_compactor.is_running.load(Ordering::SeqCst) {
            lmdb::compact(env.clone())?;
        }

        let mut destination = destination.clone();

        // if destination is not ended in .mdb, append it
        match destination.extension() {
            Some(ext) if ext != "mdb" => destination.set_extension("mdb"),
            None => destination.set_extension("mdb"),
            _ => true,
        };

        match fs::copy(get_storage_path().join(MDB_FILE_NAME), destination) {
            Ok(bytes_copied) => debug!("Map exported: {} bytes", bytes_copied),
            Err(e) => warn!("Failed to export map: {}", e),
        }
    }

    Ok(())
}

pub fn load_area(sector: Sector, env: Env, commands: &mut Commands, tiles: &Res<MapTiles>) {
    let item_repository = ItemsFromHeedLmdb::new(env);

    match item_repository.get_for_area(&sector) {
        Ok(area) => {
            info!("Reading area: {:?}", area.len());
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

    execute_async(async {
        // env.prepare_for_closing().wait();
        // lmdb::compact().unwrap();

        {
            time_test!("Compressing");
            compress::<Zstd>(
                lmdb::get_storage_path().join("data.mdb").to_str().unwrap(),
                Some(3),
            )
            .unwrap();
        }

        {
            time_test!("Decompressing");
            decompress::<Zstd>(
                lmdb::get_storage_path()
                    .join("data.mdb.snp")
                    .to_str()
                    .unwrap(),
            )
            .unwrap();
        }
    });

    Ok(())
}
