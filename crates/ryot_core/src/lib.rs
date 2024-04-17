#[cfg(feature = "bevy")]
pub mod cache;

pub mod prelude {
    #[cfg(feature = "bevy")]
    pub use crate::cache::{Cache, CacheSystems};
    pub use crate::Flag;
}

// TODO: Temp til we figure out module structure
pub trait Flag: Copy + Clone + Eq + PartialEq + Default + Sync + Send + 'static {
    fn is_walkable(&self) -> bool;
    fn blocks_sight(&self) -> bool;
}
