use crate::{AsyncEventApp, ConfigAsset, ConfigHandle, Configurable, LoadAssetCommand};
use bevy::asset::Assets;
use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use ryot::appearances::{ContentType, SpriteSheetSet};
use ryot::ContentConfigs;
use std::marker::PhantomData;

pub struct ContentPlugin;

impl Plugin for ContentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Sprites>()
            .add_event::<LoadAssetCommand<Content>>()
            .add_async_event::<ContentWasLoaded>()
            .add_plugins(JsonAssetPlugin::<Content>::new(&["json"]))
            .add_systems(Startup, init_content)
            .add_systems(Update, load_content)
            .add_systems(Update, handle_content_loaded);
    }
}

impl Configurable for ContentConfigs {
    fn extensions() -> Vec<&'static str> {
        vec!["content.toml"]
    }
}

#[derive(Resource, Debug, Default)]
pub struct Sprites {
    pub sheets: Option<SpriteSheetSet>,
    pub content_handle: Handle<Content>,
}

#[derive(serde::Deserialize, Asset, TypePath)]
#[serde(transparent)]
pub struct Content {
    pub content: Vec<ContentType>,
}

#[derive(Event, Debug, Clone, Default)]
pub struct ContentWasLoaded {
    pub file_name: String,
    pub content: Vec<ContentType>,
}

impl ContentWasLoaded {
    pub fn from_bytes(file_name: String, bytes: Vec<u8>) -> Option<Self> {
        if let Ok(content) = serde_json::from_slice::<Vec<ContentType>>(&bytes) {
            Some(Self { file_name, content })
        } else {
            None
        }
    }
}

pub fn init_content(mut event_writer: EventWriter<LoadAssetCommand<Content>>) {
    event_writer.send(LoadAssetCommand {
        path: "catalog-content.json".to_string(),
        _marker: PhantomData,
    });
}

pub fn load_content(
    asset_server: Res<AssetServer>,
    mut sprites: ResMut<Sprites>,
    mut reader: EventReader<LoadAssetCommand<Content>>,
) {
    for LoadAssetCommand { path, .. } in reader.read() {
        sprites.content_handle = asset_server.load::<Content>(path.clone());
    }
}

fn handle_content_loaded(
    contents: ResMut<Assets<Content>>,
    config: Res<ConfigHandle<ContentConfigs>>,
    configs: Res<Assets<ConfigAsset<ContentConfigs>>>,
    mut sprites: ResMut<Sprites>,
    mut ev_asset: EventReader<AssetEvent<Content>>,
    mut ev_content: EventReader<ContentWasLoaded>,
) {
    let Some(ConfigAsset(config)) = configs.get(config.handle.id()) else {
        warn!("No config found for content");
        return;
    };

    let mut update_sheets = |content: &[ContentType]| {
        sprites.sheets = Some(SpriteSheetSet::from_content(content, &config.sprite_sheet));
    };

    for event in ev_content.read() {
        debug!("Handling loaded content from file: {:?}", event.file_name);
        update_sheets(&event.content);
    }

    for event in ev_asset.read() {
        debug!("Handling asset event for content: {:?}", event);
        let AssetEvent::LoadedWithDependencies { id } = event else {
            continue;
        };

        if let Some(content) = contents.get(*id) {
            update_sheets(&content.content);
        }
    }
}
