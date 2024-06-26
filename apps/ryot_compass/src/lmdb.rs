use crate::{ExportMap, LoadMap};
use bevy::prelude::*;
use heed::types::Bytes;
use log::{debug, warn};
use ryot::plugins::LmdbPlugin as RyotLmdbPlugin;
use ryot::prelude::*;
use std::collections::HashMap;
use std::sync::atomic::Ordering;
use std::{cmp, fs};

pub struct LmdbPlugin;

impl Plugin for LmdbPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ExportMap>()
            .add_async_event::<LoadMap>()
            .add_plugins(RyotLmdbPlugin)
            .add_systems(
                Update,
                (
                    compact_map,
                    export_map.map(drop).run_if(on_event::<ExportMap>()),
                    (
                        load_map.map(drop),
                        init_new_map.map(drop),
                        reload_visible_area,
                    )
                        .chain()
                        .run_if(on_event::<LoadMap>()),
                    read_area_reseting_when_map_is_loaded,
                    load_tile_content.run_if(on_event::<LoadObjects>()),
                )
                    .chain()
                    .run_if(in_state(RyotContentState::Ready)),
            );
    }
}

fn read_area_reseting_when_map_is_loaded(
    mut last_area: Local<Sector>,
    mut load_map_events: EventReader<LoadMap>,
    env: ResMut<LmdbEnv>,
    tiles: Res<MapTiles<Entity>>,
    sector_query: Query<&Sector, (With<Camera>, Changed<Sector>)>,
    object_loaded_event_sender: EventWriter<LoadObjects>,
) {
    if load_map_events.read().len() > 0 {
        *last_area = Sector::default();
    }

    read_area(
        tiles,
        env,
        last_area,
        sector_query,
        object_loaded_event_sender,
    );
}

fn load_map(
    mut env: ResMut<LmdbEnv>,
    mut commands: Commands,
    mut tiles: ResMut<MapTiles<Entity>>,
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
            Ok(bytes_copied) => debug!("Map loaded: {} bytes", bytes_copied),
            Err(e) => {
                warn!("Failed to load map: {}", e);
                continue;
            }
        }
    }

    Ok(())
}

fn init_new_map(
    mut env: ResMut<LmdbEnv>,
    mut q_camera_transform: Query<&mut Transform, With<Camera>>,
) -> color_eyre::Result<()> {
    let new_env = create_env(get_storage_path()).expect("Failed to create LMDB env");
    let (wtxn, _) =
        rw::<Bytes, SerdePostcard<HashMap<Layer, Item>>>(&new_env, DatabaseName::Tiles)?;
    wtxn.commit()?;

    *q_camera_transform.single_mut() = Transform::IDENTITY;

    env.0 = Some(new_env);

    Ok(())
}

fn export_map(
    env: Res<LmdbEnv>,
    lmdb_compactor: ResMut<LmdbCompactor>,
    mut map_export_events: EventReader<ExportMap>,
) -> color_eyre::Result<()> {
    let Some(env) = &env.0 else {
        return Ok(());
    };

    for ExportMap(destination) in map_export_events.read() {
        if !lmdb_compactor.is_running.load(Ordering::SeqCst) {
            compact(env.clone())?;
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

fn load_tile_content(world: &mut World) {
    let events = world.resource::<Events<LoadObjects>>();
    let mut bundles: Vec<TiledContentBundle> = vec![];

    for LoadObjects(event_bundles) in events.get_reader().read(events) {
        bundles.extend(event_bundles);
    }

    bundles.sort_by(|a, b| match a.layer {
        layer if layer == b.layer => cmp::Ordering::Equal,
        layer if layer < b.layer => cmp::Ordering::Less,
        _ => cmp::Ordering::Greater,
    });

    for bundle in bundles {
        update(
            world,
            bundle.into(),
            (bundle.position, bundle.layer, Visibility::Visible, None),
            CommandState::default().persist(),
        );
    }
}
