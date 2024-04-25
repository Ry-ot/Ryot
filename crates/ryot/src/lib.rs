#![feature(fn_traits)]
#![feature(lazy_cell)]
#![feature(unboxed_closures)]
#![feature(let_chains)]

#[cfg(feature = "bevy")]
pub mod bevy_ryot;

pub mod prelude {
    #[cfg(feature = "bevy")]
    pub use crate::bevy_ryot::*;
    pub use ryot_internal::prelude::*;
}
