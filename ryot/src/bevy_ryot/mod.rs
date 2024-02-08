//! Bevy plugins and utilities for RyOT games.
//!
//! This module is intended to be used as a library dependency for RyOT games.
//! It provides common ways of dealing with OT content, such as loading sprites and appearances,
//! configuring the game, and handling asynchronous events.
mod appearances;

pub use appearances::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use std::marker::PhantomData;

mod async_events;
pub use async_events::*;

pub mod drawing;

pub mod sprites;

use crate::appearances::{ContentType, SpriteSheetDataSet};
use crate::bevy_ryot::sprites::SpritesToBeLoaded;
use crate::position::TilePosition;
use crate::prelude::drawing::Layer;
use crate::CONTENT_CONFIG;
use bevy::app::{App, Plugin, Update};
use bevy::asset::{Asset, Assets, Handle};
use bevy::prelude::*;
use bevy::sprite::{Anchor, MaterialMesh2dBundle};
use bevy::utils::HashMap;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_asset_loader::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use leafwing_input_manager::prelude::*;

pub static RYOT_ANCHOR: Anchor = Anchor::BottomRight;
pub static GRID_LAYER: Layer = Layer::Custom(998);

/// The states that the content loading process can be in.
/// This is used to track the progress of the content loading process.
/// It's also used to determine if the content is ready to be used.
/// It's internally used by the `ContentPlugin` and should not be manipulated directly.
/// Can be checked by applications to perform actions that depend on the state of the content.
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum InternalContentState {
    #[default]
    LoadingContent,
    PreparingContent,
    PreparingSprites,
    Ready,
}

/// An asset that holds a collection of raw content configs.
#[derive(serde::Deserialize, Asset, TypePath)]
#[serde(transparent)]
pub struct Catalog {
    pub content: Vec<ContentType>,
}

/// A trait that represents Preloaded and Content assets.
/// Most of the PreloadedAssets are not directly available to the game.
/// They are only used to prepare the ContentAssets and then discarded.
/// That's why there is a separation between PreloadedAssets and ContentAssets.
pub trait PreloadedContentAssets: PreloadedAssets + ContentAssets {}

/// A trait that represents assets that are preloaded within ryot.
/// It contains preloaded assets and mutable preparation methods for the ContentAssets.
pub trait PreloadedAssets: Resource + AssetCollection + Send + Sync + 'static {
    fn appearances(&self) -> &Handle<Appearance>;
    fn catalog_content(&self) -> &Handle<Catalog>;
    fn prepared_appearances_mut(&mut self) -> &mut PreparedAppearances;
    fn sprite_sheets(&mut self) -> &mut HashMap<String, Handle<Image>>;
    fn set_sprite_sheets_data(&mut self, sprite_sheet_set: SpriteSheetDataSet);
    fn insert_atlas_handle(&mut self, file: &str, handle: Handle<TextureAtlas>);
}

/// The main ContentAssets of a Ryot game, is prepared by preparer systems
/// during the loading process and is used by the game to access the content.
pub trait ContentAssets: Resource + Send + Sync + 'static {
    fn prepared_appearances(&self) -> &PreparedAppearances;
    fn sprite_sheet_data_set(&self) -> &Option<SpriteSheetDataSet>;
    fn get_atlas_handle(&self, file: &str) -> Option<&Handle<TextureAtlas>>;
}

/// A plugin that registers implementations of ContentAssets and loads them.
/// It inits the necessary resources and adds the necessary systems and plugins to load
/// the content assets.
///
/// It also manages the loading state of the content assets, the lifecycle of the content
/// and the events that allow lazy loading of sprites.
pub struct ContentPlugin<C: PreloadedContentAssets>(PhantomData<C>);

impl<C: PreloadedContentAssets> ContentPlugin<C> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<C: PreloadedContentAssets> Default for ContentPlugin<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: PreloadedContentAssets + Default> Plugin for ContentPlugin<C> {
    fn build(&self, app: &mut App) {
        app.init_resource::<C>()
            .init_resource::<SpritesToBeLoaded>()
            .register_type::<TilePosition>()
            .add_plugins(JsonAssetPlugin::<Catalog>::new(&["json"]))
            .add_plugins(AppearanceAssetPlugin)
            .add_loading_state(
                LoadingState::new(InternalContentState::LoadingContent)
                    .continue_to_state(InternalContentState::PreparingContent)
                    .load_collection::<C>(),
            )
            .add_systems(
                OnEnter(InternalContentState::PreparingContent),
                (prepare_content::<C>, prepare_appearances::<C>),
            )
            .add_systems(
                OnEnter(InternalContentState::PreparingSprites),
                sprites::prepare_sprites::<C>,
            )
            .add_event::<sprites::SpriteSheetTextureWasLoaded>()
            .add_systems(Update, sprites::load_sprite_sheets_from_command::<C>)
            .add_systems(Update, sprites::store_atlases_assets_after_loading::<C>)
            .add_systems(
                Update,
                (
                    sprites::load_sprite_system::<C>,
                    sprites::update_sprite_system::<C>,
                )
                    .run_if(in_state(InternalContentState::Ready)),
            );
    }
}

/// A system that prepares the content assets for use in the game.
/// It transforms the raw content configs into sprite sheet sets and stores them in
/// a way that the game can use them.
///
/// This is the last step of the content loading process, triggering the sprite loading process.
fn prepare_content<C: PreloadedContentAssets>(
    mut contents: ResMut<Assets<Catalog>>,
    mut content_assets: ResMut<C>,
    mut state: ResMut<NextState<InternalContentState>>,
) {
    debug!("Preparing content");

    let Some(catalog) = contents.get(content_assets.catalog_content().id()) else {
        panic!("No catalog loaded");
    };

    content_assets.set_sprite_sheets_data(SpriteSheetDataSet::from_content(
        &catalog.content,
        &CONTENT_CONFIG.sprite_sheet,
    ));

    state.set(InternalContentState::PreparingSprites);

    contents.remove(content_assets.catalog_content().id());

    debug!("Finished preparing content");
}

/// Quick way to create WASM compatible windows with a title.
pub fn entitled_window(title: String) -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title,
            // Bind to canvas included in `index.html`
            canvas: Some("#bevy".to_owned()),
            // The canvas size is constrained in index.html and build/web/styles.css
            fit_canvas_to_parent: true,
            // Tells wasm not to override default event handling, like F5 and Ctrl+R
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    }
}

/// Helper trait to add plugins only if they haven't been added already.
/// This is useful for external plugins that are used by multiple plugins or dependencies
/// and should only be added once.
///
/// # Example
/// You have a UI plugin dependent on Egui but you also use Bevy's inspector plugin that uses Egui.
/// You can use add_optional_plugin(EguiPlugin) in your UI plugin to avoid adding EguiPlugin twice,
/// clashing with the inspector plugin.
///
/// So instead of having
/// ```rust
/// use bevy::prelude::*;
/// use bevy::time::TimePlugin;
///
/// pub struct MyPlugin;
///
/// impl Plugin for MyPlugin {
///     fn build(&self, app: &mut App) {
///         if !app.is_plugin_added::<TimePlugin>() {
///             app.add_plugins(TimePlugin);
///         }
///
///        //...
///     }
/// }
/// ```
/// You can do
/// ```rust
/// use bevy::prelude::*;
/// use bevy::time::TimePlugin;
/// use ryot::prelude::OptionalPlugin;
///
/// pub struct MyPlugin;
///
/// impl Plugin for MyPlugin {
///     fn build(&self, app: &mut App) {
///        app.add_optional_plugin(TimePlugin);
///
///        //...
///     }
/// }
/// ```
pub trait OptionalPlugin {
    fn add_optional_plugin<T: Plugin>(&mut self, plugin: T) -> &mut Self;
}

impl OptionalPlugin for App {
    fn add_optional_plugin<T: Plugin>(&mut self, plugin: T) -> &mut Self {
        if !self.is_plugin_added::<T>() {
            self.add_plugins(plugin);
        }

        self
    }
}

/// A system to spawn a grid of lines to represent the tiles in the game using a custom color.
///
/// # Example
/// ```rust
/// use bevy::prelude::*;
/// use ryot::prelude::spawn_grid;
///
/// App::new().add_systems(Startup, spawn_grid(Color::WHITE));
/// ```
pub fn spawn_grid(
    color: Color,
) -> impl FnMut(Commands, ResMut<Assets<Mesh>>, ResMut<Assets<ColorMaterial>>) {
    move |mut commands: Commands,
          mut meshes: ResMut<Assets<Mesh>>,
          mut materials: ResMut<Assets<ColorMaterial>>| {
        let mut positions = Vec::new();
        let mut colors = Vec::new();
        let mut indices = Vec::new();
        let mut idx = 0;

        let (bottom_left_tile, top_right_tile) = (TilePosition::MIN, TilePosition::MAX);
        let (bottom_left, top_right) = (Vec2::from(bottom_left_tile), Vec2::from(top_right_tile));
        let tile_size = CONTENT_CONFIG.sprite_sheet.tile_size.as_vec2();

        for col in bottom_left_tile.x - 1..=top_right_tile.x {
            let x_offset = (col * tile_size.x as i32) as f32;

            positions.push([x_offset, bottom_left.y, 0.0]);
            positions.push([x_offset, top_right.y + tile_size.y, 0.0]);

            colors.extend(vec![color.as_rgba_f32(); 2]);

            indices.extend_from_slice(&[idx, idx + 1]);
            idx += 2;
        }

        for row in bottom_left_tile.y - 1..=top_right_tile.y {
            let y_offset = (row * tile_size.y as i32) as f32;

            positions.push([bottom_left.x - tile_size.x, y_offset, 0.0]);
            positions.push([top_right.x, y_offset, 0.0]);

            colors.extend(vec![color.as_rgba_f32(); 2]);

            indices.extend_from_slice(&[idx, idx + 1]);
            idx += 2;
        }

        let mut mesh = Mesh::new(PrimitiveTopology::LineList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        mesh.set_indices(Some(Indices::U32(indices)));

        let mesh_handle: Handle<Mesh> = meshes.add(mesh);

        commands.spawn(MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            transform: Transform::from_translation(Vec2::ZERO.extend(GRID_LAYER.z() as f32)),
            material: materials.add(ColorMaterial::default()),
            ..default()
        });
    }
}

/// A helper for leafwing-input-manager that checks if an action happened based on
/// long-press triggers. If you have a long press trigger, then it checks for pressed,
/// otherwise it checks for just_pressed.
pub fn check_action<A: Actionlike>(
    action: A,
    long_press_triggered: bool,
    action_state: &ActionState<A>,
) -> bool {
    if long_press_triggered {
        action_state.pressed(action)
    } else {
        action_state.just_pressed(action)
    }
}
