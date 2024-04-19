//! # Appearances
//! This module contains the code to load the appearances.dat file.
//! This file contains the information needed to load sprites and other content.
use crate::prelude::*;
use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetLoader, AsyncReadExt, BoxedFuture, LoadContext};
use bevy::prelude::*;
use bevy::utils::HashMap;
use ryot_assets::prelude::*;
use std::result;
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
#[derive(Debug, TypePath, Asset)]
pub struct Appearance(pub VisualElements);

/// The loader for the Appearance asset.
/// It reads the file and decodes it from protobuf.
/// See ryot::appearances::VisualElement for more information.
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
    ) -> BoxedFuture<'a, result::Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;

            Ok(Appearance(cip::from_bytes(bytes)?))
        })
    }

    fn extensions(&self) -> &[&str] {
        &["dat"]
    }
}

#[derive(Resource, Debug, Default)]
pub struct PreparedAppearances {
    groups: HashMap<AppearanceGroup, HashMap<u32, VisualElement>>,
}

impl PreparedAppearances {
    pub fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }

    pub fn insert(&mut self, group: AppearanceGroup, id: u32, visual_element: VisualElement) {
        self.groups
            .entry(group)
            .or_default()
            .insert(id, visual_element);
    }

    pub fn get_group(&self, group: AppearanceGroup) -> Option<&HashMap<u32, VisualElement>> {
        self.groups.get(&group)
    }

    pub fn get_for_group(&self, group: AppearanceGroup, id: u32) -> Option<&VisualElement> {
        self.groups.get(&group)?.get(&id)
    }
}

#[derive(Hash, Eq, Default, PartialEq, Debug, Copy, Clone, Reflect)]
pub enum AppearanceGroup {
    #[default]
    Object,
    Outfit,
    Effect,
    Missile,
}

/// Prepares the appearances from the .dat file into a HashMap to allow fast access
/// to the appearances by id. It keeps the appearances in their original separation:
/// objects, outfits, effects, missiles and special.
///
/// A prepared appearance must have at least an id and a main sprite id.
/// Appearances that don't have at least these two fields are ignored.
pub(crate) fn prepare_appearances<C: AppearanceAssets>(
    mut content_assets: ResMut<C>,
    mut appearances_assets: ResMut<Assets<Appearance>>,
) {
    debug!("Preparing appearances");
    let Appearance(appearances) = appearances_assets
        .get(content_assets.appearances())
        .expect("Appearance not found");

    let prepared_appearances = content_assets.prepared_appearances_mut();

    for (from, group) in [
        (&appearances.objects, AppearanceGroup::Object),
        (&appearances.outfits, AppearanceGroup::Outfit),
        (&appearances.effects, AppearanceGroup::Effect),
        (&appearances.missiles, AppearanceGroup::Missile),
    ] {
        process_appearances(from, |id, appearance| {
            prepared_appearances.insert(group, id, appearance.clone());
        });
    }

    appearances_assets.remove(content_assets.appearances());

    debug!("Appearances prepared");
}

fn process_appearances(
    appearances: &[VisualElement],
    mut insert_fn: impl FnMut(u32, &VisualElement),
) {
    appearances
        .iter()
        .filter_map(|appearance| {
            if appearance.sprites_info.is_empty() || appearance.id == 0 {
                return None;
            }

            Some((appearance.id, appearance))
        })
        .for_each(|(id, appearance)| insert_fn(id, appearance));
}
