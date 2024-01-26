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

mod sprites;
pub use sprites::*;

pub mod prelude {
    pub use crate::appearances::*;
    #[cfg(feature = "bevy")]
    pub use crate::bevy_ryot::*;
    #[cfg(feature = "compression")]
    pub use crate::compression::{compress, decompress, Compression, Zstd};
    pub use crate::content::*;
    #[cfg(feature = "lmdb")]
    pub use crate::lmdb::*;
    pub use crate::sprites::*;
}
