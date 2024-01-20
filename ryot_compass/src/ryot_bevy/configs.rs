/*
 * Ryot - A free and open-source MMORPG server emulator
 * Copyright (©) 2023 Lucas Grossi <lucas.ggrossi@gmail.com>
 * Repository: https://github.com/lgrossi/Ryot
 * License: https://github.com/lgrossi/Ryot/blob/main/LICENSE
 * Contributors: https://github.com/lgrossi/Ryot/graphs/contributors
 * Website: https://github.com/lgrossi/Ryot
 */
use bevy::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};
use std::any::type_name;
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Asset, Clone, Debug)]
pub struct ConfigAsset<T: Clone + Send + Sync + 'static>(pub T);

impl<'de, T: Clone + Send + Sync + 'static> Deserialize<'de> for ConfigAsset<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Directly deserialize T and place it into the `inner` field
        let inner = T::deserialize(deserializer)?;
        Ok(ConfigAsset(inner))
    }
}

impl<T: Clone + Send + Sync + 'static> TypePath for ConfigAsset<T> {
    fn type_path() -> &'static str {
        "configs::ConfigWrapper<T>"
    }

    fn short_type_path() -> &'static str {
        "ConfigWrapper<T>"
    }
}

#[derive(Resource, Clone, Debug)]
pub struct Config<T: Clone + Send + Sync + 'static> {
    pub source: String,
    pub handle: Handle<ConfigAsset<T>>,
}

#[derive(Debug, Clone, Event)]
pub struct ReloadConfig<T> {
    new_file: Option<String>,
    _marker: PhantomData<T>,
}

pub trait ConfigExtension {
    fn add_config<T: DeserializeOwned + Clone + Send + Sync + 'static>(
        &mut self,
        config_path: &str,
    ) -> &mut Self;
}

impl ConfigExtension for App {
    fn add_config<T: DeserializeOwned + Clone + Send + Sync + 'static>(
        &mut self,
        config_path: &str,
    ) -> &mut Self {
        assert!(
            !self.world.contains_resource::<Config<T>>(),
            "This config is already initialized",
        );

        self.add_event::<ReloadConfig<T>>()
            .init_asset::<ConfigAsset<T>>()
            .add_plugins(TomlAssetPlugin::<ConfigAsset<T>>::new(&["toml"]));

        let handle: Handle<ConfigAsset<T>> = self
            .world
            .get_resource::<AssetServer>()
            .unwrap()
            .load(config_path.to_string());

        self.insert_resource(Config {
            source: config_path.to_string(),
            handle,
        })
        .add_systems(Update, reload_config::<T>);

        self
    }
}

fn reload_config<T: DeserializeOwned + Clone + Send + Sync + 'static>(
    mut config: ResMut<Config<T>>,
    mut configs: ResMut<Assets<ConfigAsset<T>>>,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<ReloadConfig<T>>,
) {
    for ReloadConfig { new_file, .. } in reader.read() {
        if let Some(new_file) = &new_file && new_file != &config.source {
            if configs.contains(config.handle.id()) {
                configs.remove(config.handle.id());
            }

            config.source = new_file.clone();
            info!("Switched config '{}' file to '{}'", type_name::<T>(), new_file);
        }

        config.handle = asset_server.load(&config.source);
    }
}

pub fn print_config_system<T: Clone + Send + Sync + Debug + 'static>(
    config: Res<Config<T>>,
    configs: Res<Assets<ConfigAsset<T>>>,
) {
    if let Some(config) = configs.get(config.handle.id()) {
        info!("Config '{}': {:?}", type_name::<T>(), config);
    }
}
