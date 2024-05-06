#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use prost::{DecodeError, Message};
use ryot_core::prelude as ryot;

include!(concat!(env!("OUT_DIR"), "/tibia.rs"));

#[cfg(feature = "bevy")]
pub mod asset_loader;
pub mod conversions;

pub fn from_bytes(bytes: &[u8]) -> Result<ryot::VisualElements, DecodeError> {
    let visual_elements: VisualElements = VisualElements::decode(bytes)?;
    Ok(visual_elements.into())
}

pub mod prelude {
    pub use crate::{asset_loader::TibiaAssetsPlugin, conversions, *};
}
