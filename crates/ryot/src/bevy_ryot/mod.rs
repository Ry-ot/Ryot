//! Bevy plugins and utilities for RyOT games.
//!
//! This module is intended to be used as a library dependency for RyOT games.
//! It provides common ways of dealing with OT content, such as loading sprites,
//! configuring the game, and handling asynchronous events.
use self::sprites::SpriteMaterial;
#[cfg(feature = "debug")]
use crate::position::debug_sprite_position;
use crate::prelude::tibia::asset_loader::TibiaAssetsPlugin;
use crate::prelude::*;
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
use ryot_tiled::prelude::*;
use std::marker::PhantomData;
use strum::IntoEnumIterator;

mod game;
pub use game::*;

#[cfg(feature = "lmdb")]
pub mod lmdb;

pub mod drawing;

pub mod sprites;

pub mod perspective;

pub(crate) mod sprite_animations;
pub use sprite_animations::{toggle_sprite_animation, AnimationDuration};

pub static RYOT_ANCHOR: Anchor = Anchor::BottomRight;
pub static GRID_LAYER: Layer = Layer::Hud(0);

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

pub trait VisualElementsAsset: Resource + AssetCollection + Send + Sync + 'static {
    fn visual_elements(&self) -> &Handle<VisualElements>;
}

pub trait CatalogAsset: Resource + AssetCollection + Send + Sync + 'static {
    fn catalog_content(&self) -> &Handle<Catalog>;
}

pub trait AtlasLayoutsAsset: Resource + AssetCollection + Send + Sync + 'static {
    fn atlas_layouts(&self) -> &Vec<Handle<TextureAtlasLayout>>;
}

#[derive(Debug, Default, Clone, derive_more::Deref, derive_more::DerefMut)]
#[cfg_attr(feature = "bevy", derive(bevy::prelude::Resource))]
pub struct TextureAtlasLayouts(Vec<TextureAtlasLayout>);

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

content_plugin!(BaseContentPlugin, VisualElementsAsset);

impl<C: VisualElementsAsset + Default> Plugin for BaseContentPlugin<C> {
    fn build(&self, app: &mut App) {
        app.init_state::<InternalContentState>()
            .init_resource::<C>()
            .init_resource::<VisualElements>()
            .register_type::<TilePosition>()
            .add_plugins(TibiaAssetsPlugin)
            .add_loading_state(
                LoadingState::new(InternalContentState::LoadingContent)
                    .continue_to_state(InternalContentState::PreparingContent)
                    .load_collection::<C>(),
            )
            .add_systems(
                OnEnter(InternalContentState::PreparingContent),
                prepare_visual_elements::<C>,
            );
    }
}

content_plugin!(MetaContentPlugin, VisualElementsAsset);

impl<C: VisualElementsAsset + Default> Plugin for MetaContentPlugin<C> {
    fn build(&self, app: &mut App) {
        app.add_plugins(BaseContentPlugin::<C>::default())
            .add_systems(
                OnEnter(InternalContentState::PreparingContent),
                transition_to_ready.after(prepare_visual_elements::<C>),
            );
    }
}

content_plugin!(VisualContentPlugin, VisualElementsAsset);

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

impl<C> Plugin for VisualContentPlugin<C>
where
    C: VisualElementsAsset + CatalogAsset + AtlasLayoutsAsset + Default,
{
    fn build(&self, app: &mut App) {
        app.add_plugins(BaseContentPlugin::<C>::default())
            .configure_loading_state(
                LoadingStateConfig::new(InternalContentState::LoadingContent)
                    .with_dynamic_assets_file::<StandardDynamicAssetArrayCollection>(
                    "dynamic.atlases.ron",
                ),
            )
            .init_resource::<RectMeshes>()
            .init_resource::<SpriteMeshes>()
            .init_resource::<SpriteSheetDataSet>()
            .init_resource::<TextureAtlasLayouts>()
            .add_plugins(JsonAssetPlugin::<Catalog>::new(&["json"]))
            .add_optional_plugin(StrokedTextPlugin)
            .add_plugins(Material2dPlugin::<SpriteMaterial>::default())
            .add_systems(
                OnEnter(InternalContentState::PreparingContent),
                (
                    prepare_sprite_layouts::<C>,
                    prepare_sprite_sheets::<C>,
                    prepare_sprite_meshes,
                    transition_to_ready,
                )
                    .chain()
                    .after(prepare_visual_elements::<C>),
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
                    sprites::process_load_events_system
                        .pipe(sprites::load_sprite_system)
                        .pipe(sprites::store_loaded_appearances_system)
                        .run_if(on_event::<sprites::LoadAppearanceEvent>())
                        .in_set(SpriteSystems::Load),
                    sprites::initialize_sprite_material.in_set(SpriteSystems::Initialize),
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

/// Reads the VisualElements and stores it in the VisualElements resource, removing the asset.
/// Since VisualElements is a single resource, there is no need to keep the asset handle around,
/// allowing direct access to the VisualElements resource.
fn prepare_visual_elements<C: VisualElementsAsset>(
    content_assets: Res<C>,
    mut visual_elements_res: ResMut<VisualElements>,
    mut visual_elements_assets: ResMut<Assets<VisualElements>>,
) {
    debug!("Preparing visual elements");

    let key = content_assets.visual_elements();

    let Some(visual_elements) = visual_elements_assets.get(key) else {
        panic!("No visual elements found")
    };

    *visual_elements_res = visual_elements.clone();
    visual_elements_assets.remove(key);

    debug!("Visual elements prepared");
}

/// A system that prepares the content assets for use in the game.
/// It transforms the raw content configs into sprite sheet sets and stores them in
/// a way that the game can use them.
///
/// This is the last step of the content loading process, triggering the sprite loading process.
fn prepare_sprite_layouts<C: AtlasLayoutsAsset>(
    content_assets: Res<C>,
    mut atlas_layouts: ResMut<TextureAtlasLayouts>,
    mut atlas_layouts_assets: ResMut<Assets<TextureAtlasLayout>>,
) {
    debug!("Preparing sprite layouts");

    for (index, layout_handle) in content_assets.atlas_layouts().iter().enumerate() {
        let layout = atlas_layouts_assets
            .get(layout_handle)
            .expect("No atlas layout");

        if index == 0 {
            TILE_SIZE
                .set(layout.textures[0].size().as_uvec2())
                .expect("Failed to initialize tile size");
        }

        atlas_layouts.push(layout.clone());
        atlas_layouts_assets.remove(layout_handle);
    }

    debug!("Finished preparing sprite layouts");
}

fn prepare_sprite_sheets<C: CatalogAsset>(
    content_assets: Res<C>,
    mut contents: ResMut<Assets<Catalog>>,
    mut sprite_sheets: ResMut<SpriteSheetDataSet>,
) {
    debug!("Preparing sprite sheets");

    *sprite_sheets = contents
        .get(content_assets.catalog_content())
        .expect("No catalog loaded")
        .content
        .clone()
        .into();

    contents.remove(content_assets.catalog_content());

    debug!("Finished preparing sprite sheets");
}

fn prepare_sprite_meshes(
    mut meshes: ResMut<Assets<Mesh>>,
    mut rect_meshes: ResMut<RectMeshes>,
    mut sprite_meshes: ResMut<SpriteMeshes>,
) {
    debug!("Preparing sprite meshes");

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

    debug!("Finished preparing sprite meshes");
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
pub fn spawn_grid(
    color: Color,
) -> impl FnMut(Commands, Res<TextureAtlasLayouts>, ResMut<Assets<Mesh>>, ResMut<Assets<ColorMaterial>>)
{
    move |mut commands: Commands,
          atlas_layouts: Res<TextureAtlasLayouts>,
          mut meshes: ResMut<Assets<Mesh>>,
          mut materials: ResMut<Assets<ColorMaterial>>| {
        let mut positions = Vec::new();
        let mut colors = Vec::new();
        let mut indices = Vec::new();
        let mut idx = 0;

        let (bottom_left_tile, top_right_tile) = (TilePosition::MIN, TilePosition::MAX);
        let (bottom_left, top_right) = (Vec2::from(bottom_left_tile), Vec2::from(top_right_tile));

        let tile_size = atlas_layouts
            .get(SpriteLayout::OneByOne as usize)
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
