use crate::LoadAssetCommand;
use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetLoader, AsyncReadExt, BoxedFuture, LoadContext};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use prost::Message;
use ryot::appearances::Appearances;
use std::marker::PhantomData;
use thiserror::Error;

pub struct AppearanceAssetPlugin;

#[derive(Debug, Default, Resource)]
pub struct AppearanceHandle {
    pub handle: Handle<Appearance>,
}

impl Plugin for AppearanceAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Appearance>()
            .register_asset_loader(AppearanceAssetLoader {})
            .init_resource::<AppearanceHandle>()
            .add_event::<LoadAssetCommand<Appearance>>()
            .add_systems(Startup, init_appearances)
            .add_systems(Update, load_appearances);
    }
}

#[derive(Debug, TypeUuid, Asset, TypePath)]
#[uuid = "b34dd6e4-23de-4bd2-8375-ce64cc8ca9fd"]
pub struct Appearance(pub Appearances);

#[derive(Default)]
pub struct AppearanceAssetLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum AppearanceLoaderError {
    /// An [IO Error](std::io::Error)
    #[error("Could not read the file: {0}")]
    Io(#[from] std::io::Error),
    /// A [protobuf decode Error](prost::DecodeError)
    #[error("Could not decode from protobuf: {0}")]
    DecodeError(#[from] prost::DecodeError),
}

impl AssetLoader for AppearanceAssetLoader {
    type Asset = Appearance;
    type Settings = ();
    type Error = AppearanceLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let appearances = Appearances::decode(&*bytes)?;

            Ok(Appearance(appearances))
        })
    }

    fn extensions(&self) -> &[&str] {
        &["dat"]
    }
}

pub fn init_appearances(mut event_writer: EventWriter<LoadAssetCommand<Appearance>>) {
    event_writer.send(LoadAssetCommand {
        path: "appearances.dat".to_string(),
        _marker: PhantomData,
    });
}

fn load_appearances(
    asset_server: Res<AssetServer>,
    mut appearance: ResMut<AppearanceHandle>,
    mut reader: EventReader<LoadAssetCommand<Appearance>>,
) {
    for LoadAssetCommand { path, .. } in reader.read() {
        appearance.handle = asset_server.load(path);
    }
}
