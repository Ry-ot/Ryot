use crate::{helpers::execute_async, lmdb};
use bevy::prelude::{Deref, DerefMut, Res, ResMut, Resource, Time, Timer, TimerMode};
use heed::Env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

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
