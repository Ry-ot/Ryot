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
    groups: HashMap<AppearanceGroup, HashMap<u32, appearances::Appearance>>,
}

impl PreparedAppearances {
    pub fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }

    pub fn insert(&mut self, group: AppearanceGroup, id: u32, appearance: appearances::Appearance) {
        self.groups.entry(group).or_default().insert(id, appearance);
    }

    pub fn get_group(
        &self,
        group: AppearanceGroup,
    ) -> Option<&HashMap<u32, appearances::Appearance>> {
        self.groups.get(&group)
    }

    pub fn get_for_group(
        &self,
        group: AppearanceGroup,
        id: u32,
    ) -> Option<&appearances::Appearance> {
        self.groups.get(&group)?.get(&id)
    }
}

#[derive(Hash, Eq, Default, PartialEq, Debug, Clone)]
pub enum AppearanceGroup {
    #[default]
    Object,
    Outfit,
    Effect,
    Missile,
}

impl AppearanceGroup {
    fn label(&self) -> &'static str {
        match self {
            AppearanceGroup::Object => "object",
            AppearanceGroup::Outfit => "outfit",
            AppearanceGroup::Effect => "effect",
            AppearanceGroup::Missile => "missile",
        }
    }
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
    debug!("Preparing appearances");
    let Some(Appearance(appearances)) = appearances.get(content_assets.appearances().id()) else {
        panic!("No config found for content");
    };

    let prepared_appearances = content_assets.prepared_appearances_mut();

    for (from, group) in [
        (&appearances.object, AppearanceGroup::Object),
        (&appearances.outfit, AppearanceGroup::Outfit),
        (&appearances.effect, AppearanceGroup::Effect),
        (&appearances.missile, AppearanceGroup::Missile),
    ] {
        process_appearances(from, |id, appearance| {
            prepared_appearances.insert(group.clone(), id, appearance.clone());
        });

        debug!(
            "{} out of {} '{}' prepared",
            prepared_appearances.get_group(group.clone()).unwrap().len(),
            from.len(),
            group.label()
        );
    }

    debug!("Appearances prepared");
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

            let id = appearance.id?;

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
