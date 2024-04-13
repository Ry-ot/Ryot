//! Bevy plugins and utilities for RyOT games.
//!
//! This module is intended to be used as a library dependency for RyOT games.
//! It provides common ways of dealing with OT content, such as loading sprites and appearances,
//! configuring the game, and handling asynchronous events.
use self::sprites::SpriteMaterial;
use crate::appearances::{ContentType, SpriteSheetDataSet};
#[cfg(feature = "debug")]
use crate::position::debug_sprite_position;
use crate::position::*;
use crate::{Layer, SpriteLayout, TILE_SIZE};
use bevy::app::{App, Plugin, Update};
use bevy::asset::embedded_asset;
use bevy::asset::{Asset, Assets, Handle};
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::sprite::{Anchor, Material2dPlugin, MaterialMesh2dBundle};
use bevy::utils::HashMap;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_asset_loader::prelude::*;
use bevy_asset_loader::standard_dynamic_asset::StandardDynamicAssetArrayCollection;
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_stroked_text::StrokedTextPlugin;
use std::marker::PhantomData;
use strum::IntoEnumIterator;

mod appearances;
pub use appearances::*;

mod async_events;
pub use async_events::*;

pub mod camera;

mod conditions;
pub use conditions::*;

mod game;
pub use game::*;

#[cfg(feature = "lmdb")]
pub mod lmdb;

pub mod map;

pub mod drawing;

pub mod sprites;

pub mod perspective;

pub(crate) mod sprite_animations;

pub use sprite_animations::{toggle_sprite_animation, AnimationDuration};

pub static RYOT_ANCHOR: Anchor = Anchor::BottomRight;
pub static GRID_LAYER: Layer = Layer::Hud(0);

/// A generic cache structure leveraging `HashMap` for storing and quickly accessing data.
/// This structure is particularly useful for caching expensive computations, assets, or
/// other data for rapid retrieval.
#[derive(Resource, Default, Deref, DerefMut)]
pub struct Cache<K, V>(HashMap<K, V>);

/// Defines system sets for managing cache-related systems.
/// This enum is used to organize and control the execution order of systems that interact with
/// caches, allowing for a structured update and clean-up process.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum CacheSystems {
    UpdateCache,
    CleanCache,
}

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
    Ready,
}

/// An asset that holds a collection of raw content configs.
#[derive(serde::Deserialize, Asset, TypePath)]
#[serde(transparent)]
pub struct Catalog {
    pub content: Vec<ContentType>,
}

#[derive(Resource, Debug, Clone, Default, Deref, DerefMut)]
pub struct SpriteMeshes(pub HashMap<SpriteLayout, Handle<Mesh>>);

#[derive(Resource, Debug, Clone, Default, Deref, DerefMut)]
pub struct RectMeshes(pub HashMap<SpriteLayout, Handle<Mesh>>);

/// A trait that represents Preloaded and Content assets.
/// Most of the PreloadedAssets are not directly available to the game.
/// They are only used to prepare the ContentAssets and then discarded.
/// That's why there is a separation between PreloadedAssets and ContentAssets.
pub trait PreloadedContentAssets: PreloadedAssets + ContentAssets {}

/// A trait that represents assets that are preloaded within ryot.
/// It contains preloaded assets and mutable preparation methods for the ContentAssets.
pub trait PreloadedAssets:
    Resource + AppearanceAssets + AssetCollection + Send + Sync + 'static
{
    fn catalog_content(&self) -> Handle<Catalog>;
    fn set_sprite_sheets_data(&mut self, sprite_sheet_set: SpriteSheetDataSet);
}

/// The main ContentAssets of a Ryot game, is prepared by preparer systems
/// during the loading process and is used by the game to access the content.
pub trait ContentAssets: Resource + AppearanceAssets + Send + Sync + 'static {
    fn sprite_sheet_data_set(&self) -> Option<&SpriteSheetDataSet>;
    fn get_texture(&self, file: &str) -> Option<Handle<Image>>;
    fn get_atlas_layout(&self, layout: SpriteLayout) -> Option<Handle<TextureAtlasLayout>>;
}

pub trait AppearanceAssets: Resource + AssetCollection + Send + Sync + 'static {
    fn appearances(&self) -> Handle<Appearance>;
    fn prepared_appearances_mut(&mut self) -> &mut PreparedAppearances;
    fn prepared_appearances(&self) -> &PreparedAppearances;
}

/// A plugin that registers implementations of ContentAssets and loads them.
/// It inits the necessary resources and adds the necessary systems and plugins to load
/// the content assets.
///
/// It also manages the loading state of the content assets, the lifecycle of the content
/// and the events that allow lazy loading of sprites.
macro_rules! content_plugin {
    ($plugin_name:ident, $content_assets:tt) => {
        pub struct $plugin_name<C: $content_assets>(PhantomData<C>);

        impl<C: $content_assets> Default for $plugin_name<C> {
            fn default() -> Self {
                Self(PhantomData)
            }
        }
    };
}

content_plugin!(BaseContentPlugin, AppearanceAssets);

impl<C: AppearanceAssets + Default> Plugin for BaseContentPlugin<C> {
    fn build(&self, app: &mut App) {
        app.init_state::<InternalContentState>()
            .init_resource::<C>()
            .register_type::<TilePosition>()
            .add_plugins(AppearanceAssetPlugin)
            .add_loading_state(
                LoadingState::new(InternalContentState::LoadingContent)
                    .continue_to_state(InternalContentState::PreparingContent)
                    .load_collection::<C>(),
            )
            .add_systems(
                OnEnter(InternalContentState::PreparingContent),
                prepare_appearances::<C>,
            );
    }
}

content_plugin!(MetaContentPlugin, AppearanceAssets);

impl<C: AppearanceAssets + Default> Plugin for MetaContentPlugin<C> {
    fn build(&self, app: &mut App) {
        app.add_plugins(BaseContentPlugin::<C>::default())
            .add_systems(
                OnEnter(InternalContentState::PreparingContent),
                transition_to_ready.after(prepare_appearances::<C>),
            );
    }
}

content_plugin!(VisualContentPlugin, PreloadedContentAssets);

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum SpriteSystems {
    Load,
    Initialize,
    Update,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum AnimationSystems {
    Initialize,
    Update,
}

impl<C: PreloadedContentAssets + Default> Plugin for VisualContentPlugin<C> {
    fn build(&self, app: &mut App) {
        app.add_plugins(BaseContentPlugin::<C>::default())
            .configure_loading_state(
                LoadingStateConfig::new(InternalContentState::LoadingContent)
                    .register_dynamic_asset_collection::<StandardDynamicAssetArrayCollection>()
                    .with_dynamic_assets_file::<StandardDynamicAssetArrayCollection>(
                        "dynamic.atlases.ron",
                    ),
            )
            .init_resource::<SpriteMeshes>()
            .init_resource::<RectMeshes>()
            .add_plugins(JsonAssetPlugin::<Catalog>::new(&["json"]))
            .add_optional_plugin(StrokedTextPlugin)
            .add_plugins(Material2dPlugin::<SpriteMaterial>::default())
            .add_systems(
                OnEnter(InternalContentState::PreparingContent),
                (prepare_content::<C>, transition_to_ready)
                    .chain()
                    .after(prepare_appearances::<C>),
            )
            .init_resource::<sprite_animations::SpriteAnimationEnabled>()
            .init_resource::<sprite_animations::SynchronizedAnimationTimers>()
            .init_resource::<sprites::LoadedAppearances>()
            .add_event::<sprites::LoadAppearanceEvent>()
            .add_systems(
                Update,
                (
                    #[cfg(feature = "debug")]
                    debug_sprite_position,
                    sprites::load_from_entities_system.in_set(SpriteSystems::Load),
                    sprites::process_load_events_system::<C>
                        .pipe(sprites::load_sprite_system::<C>)
                        .pipe(sprites::store_loaded_appearances_system)
                        .run_if(on_event::<sprites::LoadAppearanceEvent>())
                        .in_set(SpriteSystems::Load),
                    sprites::ensure_appearance_initialized.in_set(SpriteSystems::Initialize),
                    sprites::update_sprite_system.in_set(SpriteSystems::Update),
                    sprite_animations::initialize_animation_sprite_system
                        .in_set(AnimationSystems::Initialize),
                    sprite_animations::tick_animation_system
                        .run_if(resource_exists_and_equals(
                            sprite_animations::SpriteAnimationEnabled(true),
                        ))
                        .in_set(AnimationSystems::Update),
                )
                    .chain()
                    .run_if(in_state(InternalContentState::Ready)),
            )
            .add_systems(
                PostUpdate,
                (
                    update_sprite_position,
                    (move_sprites_with_animation, finish_position_animation).chain(),
                ),
            );

        embedded_asset!(app, "shaders/sprite.wgsl");
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut sprite_meshes: ResMut<SpriteMeshes>,
    mut rect_meshes: ResMut<RectMeshes>,
    atlas_layouts: Res<Assets<TextureAtlasLayout>>,
) {
    debug!("Preparing content");
    let layout = content_assets
        .get_atlas_layout(SpriteLayout::OneByOne)
        .expect("OneByOne layout not found");

    let atlas_layout = atlas_layouts.get(layout).expect("No atlas layout");

    TILE_SIZE
        .set(atlas_layout.textures[0].size().as_uvec2())
        .expect("Failed to initialize tile size");

    let catalog = contents
        .get(content_assets.catalog_content())
        .expect("No catalog loaded");

    content_assets.set_sprite_sheets_data(SpriteSheetDataSet::from_content(&catalog.content));
    contents.remove(content_assets.catalog_content());
    for sprite_layout in SpriteLayout::iter() {
        sprite_meshes.insert(
            sprite_layout,
            meshes.add(Rectangle::from_size(
                sprite_layout.get_size(&tile_size()).as_vec2() * 2.,
            )),
        );
        rect_meshes.insert(
            sprite_layout,
            meshes.add(Rectangle::from_size(
                sprite_layout.get_size(&tile_size()).as_vec2(),
            )),
        );
    }

    debug!("Finished preparing content");
}

fn transition_to_ready(mut state: ResMut<NextState<InternalContentState>>) {
    state.set(InternalContentState::Ready);
}

/// Quick way to create WASM compatible windows with a title.
pub fn entitled_window(title: String) -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            name: Some(title.clone()),
            title,
            // Bind to canvas included in `index.html`
            canvas: Some("#bevy".to_owned()),
            #[cfg(target_arch = "wasm32")]
            mode: bevy::window::WindowMode::SizedFullscreen,
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

#[derive(Component, Debug, Clone)]
pub struct GridView;

/// A system to spawn a grid of lines to represent the tiles in the game using a custom color.
pub fn spawn_grid<C: ContentAssets>(
    color: Color,
) -> impl FnMut(
    Commands,
    Res<Assets<TextureAtlasLayout>>,
    Res<C>,
    ResMut<Assets<Mesh>>,
    ResMut<Assets<ColorMaterial>>,
) {
    move |mut commands: Commands,
          atlas_layouts: Res<Assets<TextureAtlasLayout>>,
          content_assets: Res<C>,
          mut meshes: ResMut<Assets<Mesh>>,
          mut materials: ResMut<Assets<ColorMaterial>>| {
        let mut positions = Vec::new();
        let mut colors = Vec::new();
        let mut indices = Vec::new();
        let mut idx = 0;

        let (bottom_left_tile, top_right_tile) = (TilePosition::MIN, TilePosition::MAX);
        let (bottom_left, top_right) = (Vec2::from(bottom_left_tile), Vec2::from(top_right_tile));

        let layout = content_assets
            .get_atlas_layout(SpriteLayout::OneByOne)
            .expect("OneByOne layout not found");

        let tile_size = atlas_layouts
            .get(layout)
            .expect("No atlas layout found")
            .textures[0]
            .size();

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

        let mut mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::default());
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        mesh.insert_indices(Indices::U32(indices));

        let mesh_handle: Handle<Mesh> = meshes.add(mesh);

        commands.spawn((
            GridView,
            MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                transform: Transform::from_translation(Vec2::ZERO.extend(GRID_LAYER.z())),
                material: materials.add(ColorMaterial::default()),
                ..default()
            },
        ));
    }
}
