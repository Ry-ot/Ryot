use crate::prelude::*;
use bevy_ecs::prelude::*;
use bevy_render::prelude::*;
use bevy_time::*;
use bevy_utils::tracing::error;
use derive_more::*;
use heed::types::Bytes;
use heed::Env;
use ryot_core::prelude::*;
use ryot_utils::prelude::execute;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Resource, Deref, DerefMut)]
pub struct LmdbEnv(pub Option<Env>);

impl Default for LmdbEnv {
    fn default() -> Self {
        Self(Some(
            create_env(get_storage_path()).expect("Failed to create LMDB env"),
        ))
    }
}

/// Resource that holds the LMDB compactor timer and a flag to indicate if the compaction is currently running.
/// The timer is set to run every 5 minutes by default.
/// It's used to control the LMDB compaction process and to avoid running it multiple times at the same time.
#[derive(Debug, Resource, Clone)]
pub struct LmdbCompactor {
    pub timer: Timer,
    pub is_running: Arc<AtomicBool>,
}

impl Default for LmdbCompactor {
    fn default() -> Self {
        Self {
            timer: Timer::new(std::time::Duration::from_secs(5 * 60), TimerMode::Repeating),
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }
}

/// This system will compact the LMDB database every X minutes, as defined by the `LmdbCompactor` resource.
/// It will only run if the previous compaction has finished.
/// This system is meant to be run in the background, it dispatches an async task to do the compaction.
///
/// Compacting is necessary to free up space in the database, as it will grow over time. LMDB does not
/// automatically free up space when data is deleted or altered, so it's necessary to run a compaction
/// every now and then to free up space and guarantee that the db file does not grow indefinitely.
pub fn compact_map(time: Res<Time>, env: Res<LmdbEnv>, mut lmdb_compactor: ResMut<LmdbCompactor>) {
    if !lmdb_compactor.timer.tick(time.delta()).finished() {
        return;
    }

    let Some(env) = &env.0 else {
        return;
    };

    let env_clone = env.clone();
    let is_running = lmdb_compactor.is_running.clone();

    execute(async move {
        let can_run = is_running
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::Relaxed)
            .is_ok();

        if can_run {
            compact(env_clone).unwrap();
            is_running.store(false, Ordering::SeqCst);
        }
    });
}

/// This system loads from LMDB the area around the camera, based on the camera's position.
/// It keeps track of the last loaded area and only loads new areas when the camera has moved
/// and contains new tiles.
pub fn read_area(
    tiles: Res<MapTiles<Entity>>,
    env: ResMut<LmdbEnv>,
    mut last_area: Local<Sector>,
    sector_query: Query<&Sector, (With<Camera>, Changed<Sector>)>,
    mut object_loaded_event_sender: EventWriter<LoadObjects>,
) {
    let Some(env) = &env.0 else {
        return;
    };

    let Ok(sector) = sector_query.get_single() else {
        return;
    };

    let sector = *sector * 1.5;

    for area in *last_area - sector {
        load_area(env.clone(), area, &tiles, &mut object_loaded_event_sender);
    }

    *last_area = sector;
}

/// This system reloads the visible area of the camera from LMDB.
pub fn reload_visible_area(
    tiles: Res<MapTiles<Entity>>,
    env: ResMut<LmdbEnv>,
    sector_query: Query<&Sector, With<Camera>>,
    mut object_loaded_event_sender: EventWriter<LoadObjects>,
) {
    let Some(env) = &env.0 else {
        return;
    };

    for sector in sector_query.iter() {
        load_area(
            env.clone(),
            *sector,
            &tiles,
            &mut object_loaded_event_sender,
        );
    }
}

/// This helper function will load the area from the LMDB database and draw it on the screen.
/// It will only run if the area is not already loaded and it will emit `ObjectsWereLoaded`
/// with the GameObjectBundle of all the elements that were loaded in each tile + layer combination.
/// This can be used by different systems in Bevy to interact with Lmdb loading.
pub fn load_area(
    env: Env,
    sector: Sector,
    tiles: &Res<MapTiles<Entity>>,
    object_loaded_event_sender: &mut EventWriter<LoadObjects>,
) {
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

                    bundles.push(GameObjectBundle::new(
                        ContentId::Object(item.id as u32),
                        tile.position,
                        layer,
                    ));
                }
            }

            object_loaded_event_sender.send(LoadObjects(bundles));
        }
        Err(e) => {
            error!("Failed to read area: {}", e);
        }
    }
}

pub fn init_tiles_db(lmdb_env: Res<LmdbEnv>) -> color_eyre::Result<()> {
    let Some(env) = &lmdb_env.0 else {
        return Ok(());
    };

    let (wtxn, _) = rw::<Bytes, SerdePostcard<HashMap<Layer, Item>>>(env, DatabaseName::Tiles)?;

    wtxn.commit()?;

    Ok(())
}
