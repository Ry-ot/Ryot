#[cfg(feature = "bevy")]
pub mod cache;

pub mod async_task;

pub mod sprite_layout;

pub mod prelude {
    pub use crate::{
        async_task::execute,
        is_true,
        sprite_layout::{SpriteLayout, SpriteLayoutIter},
        Flag,
    };

    #[cfg(feature = "bevy")]
    pub use crate::cache::{Cache, CacheSystems};
}

pub fn is_true(value: Option<bool>) -> bool {
    value == Some(true)
}

// TODO: Temp til we figure out module structure
pub trait Flag: Copy + Clone + Eq + PartialEq + Default + Sync + Send + 'static {
    fn is_walkable(&self) -> bool;
    fn blocks_sight(&self) -> bool;
}

#[cfg(test)]
mod tests;
