mod serde;
pub use serde::SerdePostcard;

use heed::{CompactionOption, Env, EnvOpenOptions, Error, RoTxn, RwTxn};
use std::fs;
use std::path::{Path, PathBuf};

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

pub const MDB_FILE_NAME: &str = "data.mdb";
pub fn create_env(path: PathBuf) -> heed::Result<Env> {
    fs::create_dir_all(path)?;

    let env = EnvOpenOptions::new()
        .max_dbs(20)
        .map_size(10 * 1024 * 1024 * 1024) // 10 GB
        .open(get_storage_path().as_path())?;

    Ok(env)
}

pub fn rw<K: 'static, V: 'static>(
    env: &Env,
    name: DatabaseName,
) -> heed::Result<(RwTxn, heed::Database<K, V>)> {
    let mut wtxn = env.write_txn()?;
    let db = env.create_database::<K, V>(&mut wtxn, Some(name.get_name()))?;
    Ok((wtxn, db))
}

pub fn ro<K: 'static, V: 'static>(
    env: &Env,
    name: DatabaseName,
) -> heed::Result<(RoTxn, heed::Database<K, V>)> {
    let rtxn = env.read_txn()?;
    let db = env.open_database::<K, V>(&rtxn, Some(name.get_name()))?;

    match db {
        Some(db) => Ok((rtxn, db)),
        None => Err(Error::InvalidDatabaseTyping),
    }
}

pub fn compact() -> heed::Result<()> {
    let env = create_env(get_storage_path())?;
    let backup_path = get_storage_path().join(MDB_FILE_NAME.to_string() + ".bkp");
    let old_path = get_storage_path().join(MDB_FILE_NAME);

    env.copy_to_file(backup_path.clone(), CompactionOption::Enabled)?;

    env.prepare_for_closing().wait();
    fs::remove_file(old_path.clone()).expect("Failed to remove the database");
    fs::rename(backup_path, old_path).expect("Failed to rename the database");

    Ok(())
}

pub fn get_storage_path() -> PathBuf {
    Path::new("target").join("storage")
}
