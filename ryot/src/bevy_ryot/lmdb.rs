use crate::lmdb;
use bevy::prelude::{Deref, DerefMut, Resource};
use heed::Env;

#[derive(Resource, Deref, DerefMut)]
pub struct LmdbEnv(pub Env);

impl Default for LmdbEnv {
    fn default() -> Self {
        Self(lmdb::create_env(lmdb::get_storage_path()).expect("Failed to create LMDB env"))
    }
}
