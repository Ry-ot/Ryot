pub mod content;
pub mod visual_elements;

#[cfg(test)]
mod tests;

#[cfg(feature = "bevy")]
pub trait AssetResource = bevy_ecs::prelude::Resource
    + bevy_asset_loader::asset_collection::AssetCollection
    + Send
    + Sync
    + 'static;
