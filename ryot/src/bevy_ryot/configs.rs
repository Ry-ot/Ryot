//! Configs plugin for Bevy.
//! It provides a way to load configuration files into Bevy's asset system.
use bevy::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};
use std::fmt::Debug;
use std::marker::PhantomData;

/// A trait that represents a configurable item.
/// It's used to load configuration files into Bevy's asset system.
/// It requires the type to compatible with Serde and Safely sent between threads.
/// It defines the file extensions that are compatible with the type.
/// In bevy, file extensions are used to determine which loader to use and are unique.
/// This means that if you have two configuration files with the same extension,
/// Bevy will complain about type conflicts.
///
/// Only .toml config files are supported at the moment.
pub trait Configurable: DeserializeOwned + Default + Clone + Send + Sync + 'static {
    fn extensions() -> Vec<&'static str>;
}

/// A wrapper around a configurable type to make it a Bevy asset.
#[derive(Asset, Clone, Debug, Default)]
pub struct ConfigAsset<T: Configurable>(pub T);

impl<'de, T: Configurable> Deserialize<'de> for ConfigAsset<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Directly deserialize T and place it into the `inner` field
        let inner = T::deserialize(deserializer)?;
        Ok(ConfigAsset(inner))
    }
}

/// A plugin that registers a Bevy asset and a toml loader for a configurable type.
pub struct ConfigPlugin<T: Configurable> {
    _marker: PhantomData<T>,
}

impl<T: Configurable> Plugin for ConfigPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_asset::<ConfigAsset<T>>()
            .add_plugins(TomlAssetPlugin::<ConfigAsset<T>>::new(&T::extensions()));
    }
}

impl<T: Configurable> Default for ConfigPlugin<T> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

/// A helper trait to get the default value of a configurable type.
/// It has a default implementation for `Option<&ConfigAsset<T>>` that returns the default value
/// of the configurable type if the option is `None`.
/// This is useful to get the value of a configurable type from the asset system, that always
/// returns an option. This way you can have a default setting to avoid breaking your app
/// if there are missing configuration files. For example:
/// ```rust
/// use bevy::prelude::*;
/// use ryot::prelude::*;
///
/// fn system<C: ContentAssets>(
///     content: Res<C>,
///     configs: Res<Assets<ConfigAsset<ContentConfigs>>>,
/// ) {
///    let config = configs.get(content.config()).or_default();
///    // do something with config //
/// }
/// ```
pub trait DefaultConfig<'a, T: Default + Clone> {
    fn or_default(self) -> T;
}

impl<'a, T: Configurable> DefaultConfig<'a, T> for Option<&'a ConfigAsset<T>> {
    fn or_default(self) -> T {
        match self {
            Some(ConfigAsset(config)) => config.clone(),
            None => T::default(),
        }
    }
}

impl<T: Configurable> TypePath for ConfigAsset<T> {
    fn type_path() -> &'static str {
        "configs::ConfigWrapper<T>"
    }

    fn short_type_path() -> &'static str {
        "ConfigWrapper<T>"
    }
}
