use bevy::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;
use ryot::ContentConfigs;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};
use std::fmt::Debug;
use std::marker::PhantomData;

pub struct ConfigPlugin<T: Configurable> {
    _marker: PhantomData<T>,
}

impl<T: Configurable> Default for ConfigPlugin<T> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

pub trait Configurable: DeserializeOwned + Default + Clone + Send + Sync + 'static {
    fn extensions() -> Vec<&'static str>;
}

impl<T: Configurable> Plugin for ConfigPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_asset::<ConfigAsset<T>>()
            .add_plugins(TomlAssetPlugin::<ConfigAsset<T>>::new(&T::extensions()));
    }
}

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

impl Configurable for ContentConfigs {
    fn extensions() -> Vec<&'static str> {
        vec!["content.toml"]
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
