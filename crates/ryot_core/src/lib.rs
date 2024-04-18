#[cfg(feature = "bevy")]
pub mod cache;

pub mod async_task;
#[cfg(feature = "compression")]
pub mod compression;

pub mod prelude {
    pub use crate::{async_task::execute, is_true, Flag};

    #[cfg(feature = "bevy")]
    pub use crate::cache::{Cache, CacheSystems};

    #[cfg(feature = "compression")]
    pub use crate::compression::{compress, decompress, Compression, Zstd};
}

pub fn is_true(value: Option<bool>) -> bool {
    value == Some(true)
}

// TODO: Temp til we figure out module structure
pub trait Flag: Copy + Clone + Eq + PartialEq + Default + Sync + Send + 'static {
    fn is_walkable(&self) -> bool;
    fn blocks_sight(&self) -> bool;
}
