//! # Appearances
//! This module contains the code to load the appearances.dat file.
//! This file contains the information needed to load sprites and other content.
use crate::appearances::{self, FixedFrameGroup};
use crate::appearances::{AppearanceFlags, Appearances, FrameGroup};
use crate::prelude::{drawing::*, *};
use bevy::asset::io::Reader;
use bevy::asset::{Asset, AssetLoader, AsyncReadExt, BoxedFuture, LoadContext};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::utils::HashMap;
use prost::Message;
use serde::{Deserialize, Serialize};
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

#[derive(Component, Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum CardinalDirection {
    North,
    East,
    #[default]
    South,
    West,
}

#[derive(Component, Debug, Copy, Clone, Default, PartialEq)]
pub struct AppearanceDescriptor {
    pub group: AppearanceGroup,
    pub id: u32,
    pub frame_group_index: FixedFrameGroup,
}

impl AppearanceDescriptor {
    pub fn new(group: AppearanceGroup, id: u32, frame_group_index: FixedFrameGroup) -> Self {
        Self {
            group,
            id,
            frame_group_index,
        }
    }

    pub fn object(id: u32) -> Self {
        Self::new(AppearanceGroup::Object, id, FixedFrameGroup::default())
    }
}

#[derive(Component, Debug, Clone, Default)]
pub(crate) struct LoadingAppearance;

#[derive(Debug, Clone, Default)]
pub struct PreparedAppearance {
    pub id: u32,
    pub name: String,
    pub layer: Layer,
    pub main_sprite_id: u32,
    pub frame_groups: Vec<FrameGroup>,
    pub flags: Option<AppearanceFlags>,
}

impl From<appearances::Appearance> for Option<PreparedAppearance> {
    fn from(item: appearances::Appearance) -> Self {
        let id = item.id?;
        let main_frame = item.frame_group.first()?.clone();
        let main_sprite_id = *main_frame.sprite_info?.sprite_id.first()?;

        Some(PreparedAppearance {
            id: item.id?,
            name: item.name.unwrap_or(id.to_string()),
            layer: Layer::from(item.flags.clone()),
            main_sprite_id,
            frame_groups: item.frame_group.clone(),
            flags: item.flags.clone(),
        })
    }
}

#[derive(Debug, Default)]
pub struct PreparedAppearances {
    groups: HashMap<AppearanceGroup, HashMap<u32, PreparedAppearance>>,
}

impl PreparedAppearances {
    pub fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }

    pub fn insert(&mut self, group: AppearanceGroup, id: u32, appearance: appearances::Appearance) {
        let prepared: Option<PreparedAppearance> = appearance.into();

        if let Some(prepared) = prepared {
            self.groups.entry(group).or_default().insert(id, prepared);
        };
    }

    pub fn get_group(&self, group: AppearanceGroup) -> Option<&HashMap<u32, PreparedAppearance>> {
        self.groups.get(&group)
    }

    pub fn get_for_group(&self, group: AppearanceGroup, id: u32) -> Option<&PreparedAppearance> {
        self.groups.get(&group)?.get(&id)
    }
}

#[derive(Hash, Eq, Default, PartialEq, Debug, Copy, Clone)]
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
pub(crate) fn prepare_appearances<C: PreloadedContentAssets>(
    mut content_assets: ResMut<C>,
    mut appearances_assets: ResMut<Assets<Appearance>>,
) {
    debug!("Preparing appearances");
    let Some(Appearance(appearances)) = appearances_assets.get(content_assets.appearances().id())
    else {
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
            prepared_appearances.insert(group, id, appearance.clone());
        });

        debug!(
            "{} out of {} '{}' prepared",
            prepared_appearances.get_group(group).unwrap().len(),
            from.len(),
            group.label()
        );
    }

    appearances_assets.remove(content_assets.appearances().id());

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
