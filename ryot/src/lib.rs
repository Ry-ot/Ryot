pub mod appearances;

#[cfg(feature = "bevy")]
pub mod bevy_ryot;

#[cfg(feature = "compression")]
mod compression;
#[cfg(feature = "compression")]
pub use compression::{compress, decompress, Compression, Zstd};

pub mod content;
pub use content::*;

#[cfg(feature = "lmdb")]
pub mod lmdb;

mod build;
pub mod sprites;

pub use sprites::*;

pub mod prelude {
    #[cfg(feature = "bevy")]
    pub use crate::bevy_ryot::*;
    pub use crate::build::*;
    #[cfg(feature = "compression")]
    pub use crate::compression::{compress, decompress, Compression, Zstd};
    pub use crate::content::*;
    #[cfg(feature = "lmdb")]
    pub use crate::lmdb::*;
    pub use crate::sprites::*;
}
