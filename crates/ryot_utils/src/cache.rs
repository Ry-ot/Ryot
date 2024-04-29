use bevy_ecs::prelude::*;
use bevy_utils::HashMap;
use derive_more::*;
use std::sync::{Arc, RwLock};

/// A generic cache structure leveraging `HashMap` for storing and quickly accessing data.
/// This structure is particularly useful for caching expensive computations, assets, or
/// other data for rapid retrieval.
///
/// This cache is not thread-safe and is intended for use where single-threaded access is guaranteed.
/// For multi-threaded access, use the [`Cache`] structure instead.
#[derive(Resource, Deref, DerefMut, Debug)]
pub struct SimpleCache<K, V>(HashMap<K, V>);

impl<K, V> Default for SimpleCache<K, V> {
    fn default() -> Self {
        SimpleCache(HashMap::new())
    }
}

/// A thread-safe, generic cache structure leveraging `Arc` and `RwLock` wrapped around a `HashMap`.
/// This cache is suitable for environments where data needs to be accessed by multiple threads concurrently.
/// For single-threaded access, use the [`SimpleCache`] structure instead.
#[derive(Resource, Deref, DerefMut, Debug)]
pub struct Cache<K, V>(Arc<RwLock<HashMap<K, V>>>);

impl<K, V> Default for Cache<K, V> {
    fn default() -> Self {
        Cache(Arc::new(RwLock::new(HashMap::new())))
    }
}

/// Defines system sets for managing cache-related systems.
/// This enum is used to organize and control the execution order of systems that interact with
/// caches, allowing for a structured update and clean-up process.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum CacheSystems {
    UpdateCache,
    CleanCache,
}
