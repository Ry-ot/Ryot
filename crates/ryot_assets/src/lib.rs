pub mod sprite_layout;

pub mod prelude {
    pub use crate::sprite_layout::{SpriteLayout, SpriteLayoutIter};
}

#[cfg(test)]
mod tests;
