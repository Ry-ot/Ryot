use bevy_ecs::prelude::*;
use bevy_utils::HashMap;
use derive_more::*;

/// A generic cache structure leveraging `HashMap` for storing and quickly accessing data.
/// This structure is particularly useful for caching expensive computations, assets, or
/// other data for rapid retrieval.
#[derive(Resource, Deref, DerefMut, Debug)]
pub struct Cache<K, V>(HashMap<K, V>);

impl<K, V> Default for Cache<K, V> {
    fn default() -> Self {
        Cache(HashMap::new())
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
