mod serde;

use std::fs;
use std::path::{Path, PathBuf};
use futures::TryFutureExt;
use heed::{Env, EnvOpenOptions, RwTxn, RoTxn};
pub use serde::{SerdePostcard};

mod compression;
pub use compression::{compress, decompress, Zstd, Compression};

#[derive(Debug, Clone, Copy)]
pub enum DatabaseName {
    Tiles,
}

impl DatabaseName {
    pub fn get_name(&self) -> &str {
        match self {
            DatabaseName::Tiles => "tiles",
        }
    }
}

#[derive(Clone)]
pub struct Lmdb {
    pub env: Env,
}

impl Lmdb {
    pub fn new() -> heed::Result<Self> {
        fs::create_dir_all(&Self::get_storage_path())?;

        let env = EnvOpenOptions::new()
            .max_dbs(20)
            .map_size(1024 * 1024 * 1024 * 1024)
            .open(Self::get_storage_path().as_path())?;

        Ok(Self { env })
    }

    pub fn rw<K: 'static, V: 'static>(&self, name: DatabaseName) -> heed::Result<(RwTxn, heed::Database<K, V>)> {
        let mut wtxn = self.env.write_txn()?;
        let db = self.env.create_database::<K, V>(&mut wtxn, Some(name.get_name()))?;
        Ok((wtxn, db))
    }

    pub fn ro<K: 'static, V: 'static>(&self, name: DatabaseName) -> heed::Result<(RoTxn, Option<heed::Database<K, V>>)> {
        let rtxn = self.env.read_txn()?;
        let db = self.env.open_database::<K, V>(&rtxn, Some(name.get_name()))?.or(None);
        Ok((rtxn, db))
    }

    pub fn init<K: 'static, V: 'static>(&self, name: DatabaseName) -> heed::Result<(RwTxn, heed::Database<K, V>)> {
        let (wtxn, _) = self.rw::<K, V>(name.clone())?;
        wtxn.commit()?;

        self.rw::<K, V>(name)
    }

    pub fn get_storage_path() -> PathBuf {
        Path::new("target").join("storage")
    }
}
