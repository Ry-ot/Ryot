//! # Appearances
//! This module contains the code to load the appearances.dat file.
//! This file contains the information needed to load sprites and other content.

use crate::appearances;
use crate::appearances::Appearances;
use crate::bevy_ryot::ContentAssets;
use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetLoader, AsyncReadExt, BoxedFuture, LoadContext};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::utils::HashMap;
use prost::Message;
use thiserror::Error;

/// A plugin to register the Appearance asset and its loader.
pub struct AppearanceAssetPlugin;

impl Plugin for AppearanceAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<Appearance>()
            .register_asset_loader(AppearanceAssetLoader {});
    }
}

/// Wrapper around the Appearances struct to make it an asset.
#[derive(Debug, TypeUuid, Asset, TypePath)]
#[uuid = "b34dd6e4-23de-4bd2-8375-ce64cc8ca9fd"]
pub struct Appearance(pub Appearances);

/// The loader for the Appearance asset.
/// It reads the file and decodes it from protobuf.
/// See ryot::appearances::Appearances for more information.
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

#[derive(Debug, Default)]
pub struct PreparedAppearances {
    pub objects: HashMap<u32, appearances::Appearance>,
    pub outfits: HashMap<u32, appearances::Appearance>,
    pub effects: HashMap<u32, appearances::Appearance>,
    pub missiles: HashMap<u32, appearances::Appearance>,
}

/// Prepares the appearances from the .dat file into a HashMap to allow fast access
/// to the appearances by id. It keeps the appearances in their original separation:
/// objects, outfits, effects, missiles and special.
///
/// A prepared appearance must have at least an id and a main sprite id.
/// Appearances that don't have at least these two fields are ignored.
pub(crate) fn prepare_appearances<C: ContentAssets>(
    mut content_assets: ResMut<C>,
    appearances: Res<Assets<Appearance>>,
) {
    info!("Preparing appearances");
    let Some(Appearance(appearances)) = appearances.get(content_assets.appearances().id()) else {
        panic!("No config found for content");
    };

    let prepared_appearances = content_assets.prepared_appearances_mut();

    for (kind, from, to) in [
        (
            "object",
            &appearances.object,
            &mut prepared_appearances.objects,
        ),
        (
            "outfit",
            &appearances.outfit,
            &mut prepared_appearances.outfits,
        ),
        (
            "effect",
            &appearances.effect,
            &mut prepared_appearances.effects,
        ),
        (
            "missile",
            &appearances.missile,
            &mut prepared_appearances.missiles,
        ),
    ] {
        process_appearances(from, |id, appearance| {
            to.insert(id, appearance.clone());
        });
        info!("{} out of {} '{}' prepared", to.len(), from.len(), kind);
    }

    info!("Appearances prepared");
}

fn process_appearances(
    appearances: &[appearances::Appearance],
    mut insert_fn: impl FnMut(u32, &appearances::Appearance),
) {
    appearances
        .iter()
        .filter_map(|appearance| {
            if appearance.frame_group.is_empty() {
                return None;
            }

            let Some(id) = appearance.id else {
                return None;
            };

            for frame_group in appearance.frame_group.iter() {
                let Some(sprite_info) = &frame_group.sprite_info else {
                    continue;
                };

                if sprite_info.sprite_id.is_empty() {
                    continue;
                }

                break;
            }

            Some((id, appearance))
        })
        .for_each(|(id, appearance)| insert_fn(id, appearance));
}
