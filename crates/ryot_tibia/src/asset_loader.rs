use crate as tibia;
use bevy_asset::io::Reader;
use bevy_asset::{AssetApp, AssetLoader, AsyncReadExt, LoadContext};
use bevy_utils::BoxedFuture;
use ryot_content::prelude::VisualElements;
use thiserror::Error;

#[derive(Default)]
struct TibiaAssetLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
enum TibiaAssetsLoaderError {
    /// An [IO Error](std::io::Error)
    #[error("Could not read the file: {0}")]
    Io(#[from] std::io::Error),
    /// A [protobuf decode Error](prost::DecodeError)
    #[error("Could not decode from protobuf: {0}")]
    DecodeError(#[from] prost::DecodeError),
}

pub struct TibiaAssetsPlugin;

impl bevy_app::Plugin for TibiaAssetsPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.init_asset::<VisualElements>()
            .register_asset_loader(TibiaAssetLoader);
    }
}

impl AssetLoader for TibiaAssetLoader {
    type Asset = VisualElements;
    type Settings = ();
    type Error = TibiaAssetsLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            Ok(tibia::from_bytes(&bytes)?)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["dat"]
    }
}
