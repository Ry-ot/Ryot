use crate::bevy_ryot::drawing::{DrawingBundle, LoadTileContent};
use crate::bevy_ryot::map::MapTiles;
use crate::bevy_ryot::AppearanceDescriptor;
use crate::lmdb::{DatabaseName, Item, ItemRepository, ItemsFromHeedLmdb, SerdePostcard};
use crate::position::Sector;
use crate::{helpers::execute_async, lmdb, Layer};
use bevy::prelude::*;
use heed::types::Bytes;
use heed::Env;
use log::error;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct LmdbPlugin;

impl Plugin for LmdbPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LmdbEnv>()
            .init_resource::<LmdbCompactor>()
            .add_systems(Startup, init_tiles_db.map(drop));
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct LmdbEnv(pub Option<Env>);

impl Default for LmdbEnv {
    fn default() -> Self {
        Self(Some(
            lmdb::create_env(lmdb::get_storage_path()).expect("Failed to create LMDB env"),
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

    execute_async(async move {
        let can_run = is_running
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::Relaxed)
            .is_ok();

        if can_run {
            lmdb::compact(env_clone).unwrap();
            is_running.store(false, Ordering::SeqCst);
        }
    });
}

/// This system loads from LMDB the area around the camera, based on the camera's position.
/// It keeps track of the last loaded area and only loads new areas when the camera has moved
/// and contains new tiles.
pub fn read_area(
    tiles: Res<MapTiles>,
    env: ResMut<LmdbEnv>,
    mut commands: Commands,
    mut last_area: Local<Sector>,
    sector_query: Query<&Sector, (With<Camera>, Changed<Sector>)>,
) {
    let Some(env) = &env.0 else {
        return;
    };

    let Ok(sector) = sector_query.get_single() else {
        return;
    };

    let sector = *sector * 1.5;

    for area in *last_area - sector {
        load_area(area, env.clone(), &mut commands, &tiles);
    }

    *last_area = sector;
}

/// This system reloads the visible area of the camera from LMDB.
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

/// This helper function will load the area from the LMDB database and draw it on the screen.
/// It will only run if the area is not already loaded and it will draw to the screen
/// using LoadTileContent events to be handled by the drawing system.
///
/// This can be used by different systems in Bevy to interact with Lmdb loading.
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

fn init_tiles_db(lmdb_env: Res<LmdbEnv>) -> color_eyre::Result<()> {
    let Some(env) = &lmdb_env.0 else {
        return Ok(());
    };

    let (wtxn, _) =
        lmdb::rw::<Bytes, SerdePostcard<HashMap<Layer, Item>>>(env, DatabaseName::Tiles)?;

    wtxn.commit()?;

    Ok(())
}
