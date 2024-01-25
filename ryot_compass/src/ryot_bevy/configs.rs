use bevy::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;
use ryot::CONTENT_CONFIG_PATH;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};
use std::any::type_name;
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
        app.add_event::<LoadConfigCommand<T>>()
            .init_resource::<ConfigHandle<T>>()
            .init_asset::<ConfigAsset<T>>()
            .add_plugins(TomlAssetPlugin::<ConfigAsset<T>>::new(&T::extensions()))
            .add_systems(Update, load_config::<T>);
    }
}

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

#[derive(Resource, Clone, Debug, Default)]
pub struct ConfigHandle<T: Clone + Send + Sync + 'static> {
    pub source: Option<String>,
    pub handle: Handle<ConfigAsset<T>>,
}

#[derive(Debug, Clone, Event)]
pub struct LoadConfigCommand<T> {
    loading_type: LoadingType<T>,
    _marker: PhantomData<T>,
}

impl<T> LoadConfigCommand<T> {
    pub fn reload() -> Self {
        Self {
            loading_type: LoadingType::Reload,
            _marker: PhantomData,
        }
    }

    pub fn load(new_file: String) -> Self {
        Self {
            loading_type: LoadingType::Load(new_file),
            _marker: PhantomData,
        }
    }

    pub fn update(new_config: T) -> Self {
        Self {
            loading_type: LoadingType::Update(new_config),
            _marker: PhantomData,
        }
    }
}

#[derive(Debug, Clone)]
pub enum LoadingType<T> {
    Reload,
    Load(String),
    Update(T),
}

impl<T> Default for LoadConfigCommand<T> {
    fn default() -> Self {
        Self {
            loading_type: LoadingType::Reload,
            _marker: PhantomData,
        }
    }
}

pub trait ConfigApp {
    fn add_config<T: Configurable>(&mut self, config_path: &str) -> &mut Self;
}

impl ConfigApp for App {
    fn add_config<T: Configurable>(&mut self, config_path: &str) -> &mut Self {
        assert!(
            !self.is_plugin_added::<ConfigPlugin<T>>(),
            "This config is already initialized",
        );

        self.add_plugins(ConfigPlugin::<T> {
            _marker: PhantomData,
        });

        let handle: Handle<ConfigAsset<T>> = self
            .world
            .get_resource::<AssetServer>()
            .unwrap()
            .load(config_path.to_string());

        self.insert_resource(ConfigHandle {
            source: Some(config_path.to_string()),
            handle,
        })
    }
}

fn load_config<T: Configurable>(
    asset_server: Res<AssetServer>,
    mut config: ResMut<ConfigHandle<T>>,
    mut configs: ResMut<Assets<ConfigAsset<T>>>,
    mut reader: EventReader<LoadConfigCommand<T>>,
) {
    for LoadConfigCommand { loading_type, .. } in reader.read() {
        match loading_type {
            LoadingType::Reload => {
                let Some(source) = &config.source else {
                    warn!(
                        "Trying to reload config for '{}', but no config file is configured",
                        type_name::<T>()
                    );
                    return;
                };

                config.handle = asset_server.load(source);
                debug!("Reloaded config '{}'", type_name::<T>());
            }
            LoadingType::Load(new_file) => {
                config.source = Some(new_file.clone());
                config.handle = asset_server.load(new_file.clone());
                debug!("Loading config '{}' from '{}'", type_name::<T>(), new_file);
            }
            LoadingType::Update(new_config) => {
                config.source = None;
                config.handle = configs.add(ConfigAsset(new_config.clone()));
                debug!("Setting config '{}' dynamically", type_name::<T>());
            }
        }
    }
}

pub fn print_config_system<T: Clone + Send + Sync + Debug + 'static>(
    config: Res<ConfigHandle<T>>,
    configs: Res<Assets<ConfigAsset<T>>>,
) {
    if let Some(config) = configs.get(config.handle.id()) {
        debug!("Config '{}': {:?}", type_name::<T>(), config);
    }
}

pub fn test_reload_config<T: Default + Clone + Send + Sync + 'static>(
    keyboard_input: Res<Input<KeyCode>>,
    mut writer: EventWriter<LoadConfigCommand<T>>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        writer.send(LoadConfigCommand::<T>::reload());
    }

    if keyboard_input.just_pressed(KeyCode::L) {
        writer.send(LoadConfigCommand::<T>::load(
            CONTENT_CONFIG_PATH.to_string(),
        ));
    }

    if keyboard_input.just_pressed(KeyCode::N) {
        writer.send(LoadConfigCommand::<T>::update(T::default()));
    }
}
