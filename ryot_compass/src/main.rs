use bevy::app::AppExit;

use bevy::reflect::TypeUuid;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle};
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use std::fmt::Debug;

use bevy_egui::{EguiContexts, EguiPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use egui::load::SizedTexture;
use egui::TextureId;
use itertools::Itertools;

mod error_handling;
mod helpers;
use error_handling::{check_for_exit, display_error_window, ErrorState};
use helpers::camera::movement as camera_movement;
use ryot::*;

// #[cfg(all(feature = "lmdb", not(target_arch = "wasm32")))]
// use ryot_compass::item::ItemsFromHeedLmdb;

// #[cfg(all(feature = "lmdb", not(target_arch = "wasm32")))]
// use ryot_compass::lmdb::LmdbEnv;

use ryot_compass::{
    draw_palette_window, draw_sprite, load_sprites, test_reload_config, Appearance,
    AppearanceAssetPlugin, AppearanceHandle, ConfigExtension, EventSender, LoadedSprite, Palette,
    PaletteState, TextureAtlasHandlers, Tile, TilesetCategory,
};
use winit::window::Icon;

use bevy::asset::AssetMetaCheck;
use rfd::AsyncFileDialog;

use ryot::tile_grid::TileGrid;
use ryot_compass::content::{ContentPlugin, ContentWasLoaded, Sprites};
use std::future::Future;

// fn scroll_events(mut minimap: ResMut<Minimap>, mut scroll_evr: EventReader<MouseWheel>) {
//     for ev in scroll_evr.read() {
//         minimap.zoom += ev.y * 0.1;
//         minimap.zoom = minimap.zoom.clamp(1.0, 25.0);
//     }
// }
//
// fn draw_tiles_on_minimap(
//     mut minimap: ResMut<Minimap>,
//     mut images: ResMut<Assets<Image>>,
//     mut tiles: ResMut<Tiles>,
// ) {
//     let positions = tiles
//         .0
//         .iter()
//         .map(|(tile, _)| UVec2::new(tile.position.x.into(), tile.position.y.into()))
//         .collect::<Vec<_>>();
//     minimap.update_texture(positions, &mut images);
//     tiles.0.clear();
// }

#[derive(AsBindGroup, TypeUuid, Asset, TypePath, Debug, Clone)]
#[uuid = "f229fdae-d598-45ac-8225-97e2a3f011e0"]
pub struct RainbowOutlineMaterial {
    /// The thickness of the outline. Preferred values between 0.01 and 0.005.
    #[uniform(0)]
    pub thickness: f32,
    /// Frequency at which the colors of the rainbow are iterated over.
    #[uniform(0)]
    pub frequency: f32,
    /// The texture to outline.
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
}

impl Material2d for RainbowOutlineMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/rainbow_outline_material.wgsl".into()
    }
}

fn spawn_camera(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // mut rb_materials: ResMut<Assets<RainbowOutlineMaterial>>,
) {
    let TileGrid {
        columns,
        rows,
        tile_size,
    } = TileGrid::default();

    let mut positions = Vec::new();
    let mut colors = Vec::new();
    let mut indices = Vec::new();

    let width = (columns * tile_size.x) as f32;
    let height = (rows * tile_size.y) as f32;

    // Create vertices for the grid
    for i in 0..=columns {
        let offset = (i * tile_size.x) as f32;

        // Horizontal line
        positions.push([0.0, -offset, 0.0]);
        positions.push([width, -offset, 0.0]);
        // Vertical line
        positions.push([offset, 0.0, 0.0]);
        positions.push([offset, -height, 0.0]);

        // Add colors (white for grid lines)
        colors.extend(vec![Color::WHITE.as_rgba_f32(); 4]);

        // Add indices
        let idx = i * 4;
        indices.extend_from_slice(&[idx, idx + 1, idx + 2, idx + 3]);
    }

    // Create the mesh
    let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.set_indices(Some(Indices::U32(indices)));

    let mesh_handle: Handle<Mesh> = meshes.add(mesh);

    // Spawn camera
    commands.spawn(Camera2dBundle::default());

    // Spawn a black square on top for the main area
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgba(255., 255., 255., 0.01),
            custom_size: Some(Vec2::new(width, height)),
            ..Default::default()
        },
        transform: Transform::from_xyz(width / 2., height / -2., 0.), // Slightly in front to cover the white border
        ..Default::default()
    });

    // Spawn the square with the grid
    commands.spawn(MaterialMesh2dBundle {
        mesh: mesh_handle.into(),
        transform: Transform::from_translation(Vec3::new(0., 0., 256.)),
        material: materials.add(ColorMaterial::default()),
        ..default()
    });

    commands.spawn(SpriteBundle {
        texture: asset_server.load("ryot_mascot.png"),
        transform: Transform::from_translation(Vec3::new(
            (columns / 2) as f32,
            (rows / 2) as f32,
            1.,
        ))
        .with_scale(Vec3::splat(16.)),
        ..Default::default()
    });

    // commands.spawn(MaterialMesh2dBundle {
    //     mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
    //     transform: Transform::from_translation(Vec3::new(64., 75., 0.))
    //         .with_scale(Vec3::splat(128.)),
    //     material: rb_materials.add(RainbowOutlineMaterial {
    //         thickness: 0.01,
    //         frequency: 1.0,
    //         texture: asset_server.load("ryot_mascot.png"),
    //     }),
    //     ..default()
    // });
}

#[derive(Resource, Debug)]
pub struct CursorPos(Vec2);

impl Default for CursorPos {
    fn default() -> Self {
        // Initialize the cursor pos at some far away place. It will get updated
        // correctly when the cursor moves.
        Self(Vec2::new(0.0, 0.0))
    }
}

#[allow(dead_code)]
#[derive(Resource, Debug, Default)]
pub struct Tiles(Vec<(Tile, bool)>);

// #[cfg(all(feature = "lmdb", not(target_arch = "wasm32")))]
// fn load_tiles(env: ResMut<LmdbEnv>, mut tiles: ResMut<Tiles>) {
//     let tiles = &mut tiles.0;
//
//     if tiles.len() > 0 {
//         return;
//     }
//
//     time_test!("Loading");
//
//     let initial_pos = Position::new(60000, 60000, 0);
//     let final_pos = Position::new(61100, 61100, 0);
//
//     let item_repository = ItemsFromHeedLmdb::new(env.0.clone().unwrap());
//
//     let lmdb_tiles = {
//         // time_test!("Reading");
//         item_repository
//             .get_for_area(&initial_pos, &final_pos)
//             .unwrap()
//     };
//
//     for tile in lmdb_tiles {
//         tiles.push((
//             Tile {
//                 position: Position::from_binary_key(&tile.0),
//                 item: Some(tile.1),
//             },
//             false,
//         ));
//     }
// }

// fn aaa(
//     mut commands: Commands,
//     mut egui_ctx: EguiContexts,
//     sprites: ResMut<Sprites>,
//     asset_server: Res<AssetServer>,
//     mut atlas_handlers: ResMut<TextureAtlasHandlers>,
//     mut texture_atlases: ResMut<Assets<TextureAtlas>>,
//     cursor_pos: Res<CursorPos>,
//     palette_state: Res<PaletteState>,
//     mouse_button_input: Res<Input<MouseButton>>,
//     error_states: Res<ErrorState>,
// ) {
// }

#[allow(clippy::too_many_arguments)]
fn draw(
    mut commands: Commands,
    mut egui_ctx: EguiContexts,
    sprites: Res<Sprites>,
    asset_server: Res<AssetServer>,
    mut atlas_handlers: ResMut<TextureAtlasHandlers>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    cursor_pos: Res<CursorPos>,
    palette_state: Res<PaletteState>,
    mouse_button_input: Res<Input<MouseButton>>,
    error_states: Res<ErrorState>,
) {
    if egui_ctx.ctx_mut().is_pointer_over_area() {
        return;
    }

    if error_states.has_error {
        return;
    }

    let Some(sprite_sheets) = &sprites.sheets else {
        return;
    };

    let Some(sprite_id) = palette_state.selected_tile else {
        return;
    };

    let sprites = load_sprites(
        &[sprite_id],
        sprite_sheets,
        &asset_server,
        &mut atlas_handlers,
        &mut texture_atlases,
    );

    let Some(sprite) = sprites.first() else {
        return;
    };

    if mouse_button_input.pressed(MouseButton::Left) {
        let pos = TileGrid::default().get_tile_pos_from_display_pos_vec2(cursor_pos.0);
        draw_sprite(Vec3::new(pos.x, pos.y, 1.1), sprite, &mut commands);
        debug!("Tile: {:?} drawn", pos);
    }

    if mouse_button_input.just_pressed(MouseButton::Right) {
        for x in 0..200 {
            for y in 0..120 {
                let mut sprites = vec![195613];
                if x.ge(&20) && x.le(&30) && y.ge(&20) && y.le(&30) {
                    sprites.push(91267);
                }

                let sprites = load_sprites(
                    &sprites,
                    sprite_sheets,
                    &asset_server,
                    &mut atlas_handlers,
                    &mut texture_atlases,
                );

                for (i, sprite) in sprites.iter().enumerate() {
                    draw_sprite(
                        Vec3::new(x as f32, y as f32, 1. + i as f32),
                        sprite,
                        &mut commands,
                    );
                }
            }
        }
    }

    // let loaded_monster = load_sprites(&vec![91267], &content.raw_content, &asset_server, &mut atlas_handlers, &mut texture_atlases);
    // if let Some(sprite) = loaded_monster.first() {
    //     for x in 20..30 {
    //         for y in 20..30 {
    //             draw_sprite(Vec3::new(x as f32, y as f32, 0.0), sprite, &mut commands);
    //         }
    //     }
    // }

    // let num_of_sprites = 400_689;
    // let sprites_per_row = (num_of_sprites as f32).sqrt() as u32;
    //
    // commands.spawn_batch((0..num_of_sprites).map(move |i| {
    //     let x = (i % sprites_per_row) as f32 * 50.0;
    //     let y = (i / sprites_per_row) as f32 * 50.0;
    //     SpriteBundle {
    //         texture: tile_handle_square.clone(),
    //         transform: Transform::from_xyz(x, y, 0.0),
    //         ..Default::default()
    //     }
    // }));
    //     counter.0 += 100_000;
    //
    //     return;
    // }
}

// We need to keep the cursor position updated based on any `CursorMoved` events.
pub fn update_cursor_pos(
    mut cursor_pos: ResMut<CursorPos>,
    camera_q: Query<(&GlobalTransform, &Camera)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    for cursor_moved in cursor_moved_events.read() {
        // To get the mouse's world position, we have to transform its window position by
        // any transforms on the camera. This is done by projecting the cursor position into
        // camera space (world space).
        for (cam_t, cam) in camera_q.iter() {
            if let Some(pos) = cam.viewport_to_world_2d(cam_t, cursor_moved.position) {
                *cursor_pos = CursorPos(pos);
                debug!("Cursor pos: {:?}", pos);
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn update_cursor(
    sprites: Res<Sprites>,
    cursor_pos: Res<CursorPos>,
    asset_server: Res<AssetServer>,
    palette_state: Res<PaletteState>,
    mut egui_ctx: EguiContexts,
    mut windows: Query<&mut Window>,
    mut cursor_query: Query<(
        &mut Transform,
        &mut Visibility,
        &mut TextureAtlasSprite,
        &mut Handle<TextureAtlas>,
        &SelectedTile,
    )>,
    mut atlas_handlers: ResMut<TextureAtlasHandlers>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if egui_ctx.ctx_mut().is_pointer_over_area() {
        egui_ctx
            .ctx_mut()
            .set_cursor_icon(egui::CursorIcon::Default);
        windows.single_mut().cursor.icon = CursorIcon::Default;
        windows.single_mut().cursor.visible = true;
    }

    let Some(sprite_sheets) = &sprites.sheets else {
        return;
    };

    let Some(sprite_id) = palette_state.selected_tile else {
        return;
    };

    let sprites = load_sprites(
        &[sprite_id],
        sprite_sheets,
        &asset_server,
        &mut atlas_handlers,
        &mut texture_atlases,
    );

    let Some(new_sprite) = sprites.first() else {
        return;
    };

    for (mut transform, mut visibility, mut sprite, mut atlas_handle, _) in cursor_query.iter_mut()
    {
        *atlas_handle = new_sprite.atlas_texture_handle.clone();
        sprite.index = new_sprite.get_sprite_index();

        let tile_grid = TileGrid::default();
        let tile_pos = tile_grid.get_tile_pos_from_display_pos_vec2(cursor_pos.0);
        let Some(cursor_pos) = tile_grid.get_display_position_from_tile_pos_vec2(tile_pos) else {
            return;
        };

        transform.translation = Vec3::from((cursor_pos, 128.));

        if cursor_pos == Vec2::ZERO {
            *visibility = Visibility::Hidden;
        } else {
            *visibility = Visibility::Visible;
        }

        if egui_ctx.ctx_mut().is_pointer_over_area() {
            continue;
        }

        match *visibility {
            Visibility::Visible => {
                egui_ctx.ctx_mut().set_cursor_icon(egui::CursorIcon::None);
                windows.single_mut().cursor.icon = CursorIcon::Default;
                windows.single_mut().cursor.visible = false;
            }
            Visibility::Hidden => {
                egui_ctx
                    .ctx_mut()
                    .set_cursor_icon(egui::CursorIcon::NotAllowed);
                windows.single_mut().cursor.icon = CursorIcon::NotAllowed;
                windows.single_mut().cursor.visible = true;
            }
            _ => {}
        }
    }
}

fn spawn_cursor(mut commands: Commands) {
    commands.spawn((
        SpriteSheetBundle { ..default() },
        SelectedTile {
            index: None,
            atlas: None,
        },
    ));
}

fn ui_example(
    mut egui_ctx: EguiContexts,
    sprites: Res<Sprites>,
    mut exit: EventWriter<AppExit>,
    content_sender: Res<EventSender<ContentWasLoaded>>,
    mut about_me: ResMut<AboutMeOpened>,
) {
    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx_mut(), |ui| {
        egui::menu::bar(ui, |ui| {
            ui.scope(|ui| {
                let mut style = (*ui.ctx().style()).clone();

                // Modify the style for your specific widget
                style.visuals.widgets.inactive.bg_fill = egui::Color32::GRAY;
                style.visuals.widgets.active.bg_fill = egui::Color32::GRAY;
                style.visuals.widgets.hovered.bg_fill = egui::Color32::GRAY;

                // Temporarily apply the style
                ui.set_style(style);

                let is_content_loaded = sprites.sheets.is_some();

                // Load the image using `image-rs`
                // let image_data = include_bytes!("path/to/your/image.png").to_vec();
                // let image = image::RgbaImage::from_raw(1024, 1024, image_data);
                //
                // // Create an `egui::TextureHandle`
                // let texture_handle = egui::TextureHandle::from_rgba_unmultiplied(
                //     ctx,
                //     egui::ColorImage::from_rgba_unmultiplied(size, &image_data)
                // );

                // let img = egui::include_image!("../assets/icons/compass_2.png");
                //
                // ui.image(img);

                egui::menu::menu_button(ui, "File", |ui| {
                    // #[cfg(not(target_arch = "wasm32"))]
                    if ui
                        .add_enabled(is_content_loaded, egui::Button::new("🗁 Open"))
                        .clicked()
                    {
                        read_file(
                            AsyncFileDialog::new().add_filter(".mdb, .otbm", &["mdb", "otbm"]),
                            |(file_name, content)| {
                                debug!("Loading map from file: {:?}", file_name);
                                debug!("Content: {:?}", content);
                                debug!("Current dir: {:?}", std::env::current_dir());
                            },
                        );

                        // let path = rfd::FileDialog::new()
                        //     .add_filter(".mdb, .otbm", &["mdb", "otbm"])
                        //     .pick_file();
                        //
                        // debug!("Loading map from file: {:?}", path);
                        // debug!("Current dir: {:?}", std::env::current_dir());
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    if ui
                        .add_enabled(is_content_loaded, egui::Button::new("💾 Save"))
                        .clicked()
                    {
                        let path = rfd::FileDialog::new()
                            .add_filter(".mdb, .otbm", &["mdb", "otbm"])
                            .save_file();

                        debug!("Saving map to file: {:?}", path);
                    }

                    ui.separator();

                    // #[cfg(not(target_arch = "wasm32"))]
                    if ui.button("Load Content").clicked() {
                        let sender = content_sender.0.clone();

                        read_file(
                            AsyncFileDialog::new().add_filter(".json", &["json"]),
                            move |(file_name, loaded)| {
                                debug!("Loading content from file: {:?}", file_name);
                                let Some(content_was_loaded) =
                                    ContentWasLoaded::from_bytes(file_name.clone(), loaded.clone())
                                else {
                                    warn!("Failed to load content from file: {:?}", file_name);
                                    return;
                                };

                                sender
                                    .send(content_was_loaded)
                                    .expect("Failed to send content loaded event");
                            },
                        );
                    }

                    ui.add_enabled(is_content_loaded, egui::Button::new("🔃 Refresh Content"))
                        .clicked();

                    ui.separator();

                    ui.button("⚙ Settings").clicked();

                    ui.separator();

                    if ui.button("Exit").clicked() {
                        exit.send(AppExit);
                    }
                });

                egui::menu::menu_button(ui, "View", |ui| {
                    if ui.button("Windows").clicked() {
                        // Open action
                    }
                });

                egui::menu::menu_button(ui, "Help", |ui| {
                    if ui.button("About").clicked() {
                        about_me.0 = true;
                    }
                });

                // ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                //     if ui.button("⚙").clicked() {
                //     }
                // })
            });
        });
    });

    egui::Window::new("About Ryot Compass")
        .auto_sized()
        .collapsible(false)
        .movable(false)
        .default_pos(egui::pos2(100.0, 100.0)) // Adjust position as needed
        .open(&mut about_me.0)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.label("About Me information...");
        });
}

#[allow(clippy::too_many_arguments)]
pub fn print_appearances(
    sprites: Res<Sprites>,
    palette_state: ResMut<PaletteState>,
    asset_server: Res<AssetServer>,
    appearances: Res<Assets<Appearance>>,
    appearance_handle: Res<AppearanceHandle>,
    mut egui_ctx: EguiContexts,
    mut palettes: ResMut<Palette>,
    mut atlas_handlers: ResMut<TextureAtlasHandlers>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    error_states: Res<ErrorState>,
) {
    if error_states.has_error {
        return;
    }

    if !palettes
        .tile_set
        .get(&TilesetCategory::Terrains)
        .unwrap()
        .is_empty()
    {
        let Some(sprite_sheets) = &sprites.sheets else {
            return;
        };

        let mut egui_images: Vec<(LoadedSprite, egui::Image)> = vec![];

        let mut sprite_ids: Vec<u32> = if palette_state.category == TilesetCategory::Raw {
            let mut sprite_ids = vec![];

            for category_sprites in palettes.tile_set.values() {
                sprite_ids.extend(category_sprites);
            }

            sprite_ids
        } else {
            palettes
                .tile_set
                .get(&palette_state.category)
                .unwrap()
                .to_vec()
        }
        .into_iter()
        .unique()
        .collect();

        sprite_ids.sort();

        let begin = palette_state.begin().min(sprite_ids.len() - 5);
        let end = palette_state.end().min(sprite_ids.len());

        for sprite in load_sprites(
            &sprite_ids[begin..end],
            sprite_sheets,
            &asset_server,
            &mut atlas_handlers,
            &mut texture_atlases,
        ) {
            let Some(atlas) = texture_atlases.get(sprite.atlas_texture_handle.clone()) else {
                continue;
            };

            let Some(rect) = atlas.textures.get(sprite.get_sprite_index()) else {
                continue;
            };

            let uv: egui::Rect = egui::Rect::from_min_max(
                egui::pos2(rect.min.x / atlas.size.x, rect.min.y / atlas.size.y),
                egui::pos2(rect.max.x / atlas.size.x, rect.max.y / atlas.size.y),
            );

            let rect_vec2: egui::Vec2 =
                egui::Vec2::new(rect.max.x - rect.min.x, rect.max.y - rect.min.y);
            let tex: TextureId = egui_ctx.add_image(atlas.texture.clone_weak());
            egui_images.push((
                sprite,
                egui::Image::new(SizedTexture::new(tex, rect_vec2)).uv(uv),
            ));
        }

        draw_palette_window(
            sprite_ids.len(),
            palettes.tile_set.keys().sorted().collect_vec(),
            egui_images,
            egui_ctx,
            palette_state,
        );

        return;
    }

    let mut total = 0;

    if let Some(Appearance(appearances)) = appearances.get(appearance_handle.handle.clone()) {
        appearances.outfit.iter().for_each(|outfit| {
            if outfit.id.is_none() {
                warn!("No-id {:?}", outfit);
            }
            total += 1;
        });

        appearances.object.iter().for_each(|object| {
            total += 1;

            let Some(frame_group) = object.frame_group.first() else {
                warn!("No-sprite {:?}", object);
                return;
            };

            let Some(sprite_info) = &frame_group.sprite_info else {
                warn!("No-sprite {:?}", object);
                return;
            };

            let Some(sprite_id) = sprite_info.sprite_id.first() else {
                warn!("No-sprite {:?}", object);
                return;
            };

            let category: TilesetCategory = object.into();

            palettes
                .tile_set
                .get_mut(&category)
                .unwrap()
                .push(*sprite_id);
        });
    };

    total = 0;

    for (category, ids) in palettes.tile_set.iter_mut() {
        *ids = ids.iter().unique().cloned().collect();
        debug!("{}: {}", category.get_label(), ids.len());
        total += ids.len();
    }
    debug!("Total: {}", total);
}

// fn set_window_icon(
//     windows: NonSend<WinitWindows>,
//     primary_window: Query<Entity, With<PrimaryWindow>>,
// ) {
//     let primary_entity = primary_window.single();
//     let Some(primary) = windows.get_window(primary_entity) else {
//         return;
//     };
//     let icon_buf = Cursor::new(include_bytes!(
//         "../build/macos/AppIcon.iconset/icon_256x256.png"
//     ));
//     if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
//         let image = image.into_rgba8();
//         let (width, height) = image.dimensions();
//         let rgba = image.into_raw();
//         let icon = Icon::from_rgba(rgba, width, height).unwrap();
//         primary.set_window_icon(Some(icon));
//     };
// }

pub fn setup_window(
    mut egui_ctx: EguiContexts,
    windows: NonSend<WinitWindows>,
    primary_window_query: Query<Entity, With<PrimaryWindow>>,
) {
    egui_extras::install_image_loaders(egui_ctx.ctx_mut());

    let primary_window_entity = primary_window_query.single();
    let primary_window = windows.get_window(primary_window_entity).unwrap();

    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/icons/compass_4.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    primary_window.set_window_icon(Some(icon));
}

#[derive(Debug, Component)]
pub struct SelectedTile {
    pub index: Option<usize>,
    pub atlas: Option<Handle<TextureAtlas>>,
}

fn main() {
    App::new()
        .add_event::<AppExit>()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Compass".to_string(),
                        // Bind to canvas included in `index.html`
                        canvas: Some("#bevy".to_owned()),
                        // The canvas size is constrained in index.html and build/web/styles.css
                        fit_canvas_to_parent: true,
                        // Tells wasm not to override default event handling, like F5 and Ctrl+R
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            Material2dPlugin::<RainbowOutlineMaterial>::default(),
        ))
        .add_plugins(AppearanceAssetPlugin)
        .add_plugins(ContentPlugin)
        .insert_resource(ClearColor(Color::rgb(0.12, 0.12, 0.12)))
        .init_resource::<ErrorState>()
        .add_config::<ContentConfigs>(CONTENT_CONFIG_PATH)
        // .init_resource::<LmdbEnv>()
        .init_resource::<Palette>()
        .init_resource::<AboutMeOpened>()
        .init_resource::<TextureAtlasHandlers>()
        .init_resource::<CursorPos>()
        .init_resource::<Tiles>()
        .init_resource::<PaletteState>()
        .add_plugins((
            EguiPlugin,
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
            // MinimapPlugin,
        ))
        .add_systems(Startup, spawn_camera)
        // .add_systems(Startup, setup_window)
        // .add_systems(Startup, init_env.before(load_tiles))
        .add_systems(Startup, spawn_cursor)
        // .add_systems(Startup, load_tiles)
        // .add_systems(Startup, decompress_all_sprites)
        .add_systems(First, (camera_movement, update_cursor_pos).chain())
        // .add_systems(Update, decompress_all_sprites)
        .add_systems(Update, draw)
        // .add_systems(Update, draw_tiles_on_minimap)
        // .add_systems(Update, scroll_events)
        .add_systems(Update, ui_example)
        .add_systems(Update, print_appearances)
        .add_systems(Update, display_error_window)
        .add_systems(Update, check_for_exit)
        .add_systems(Update, update_cursor)
        .add_systems(Update, test_reload_config::<ContentConfigs>)
        .run();
}

#[derive(Resource, Default)]
struct AboutMeOpened(bool);

#[cfg(not(target_arch = "wasm32"))]
fn execute<F: Future<Output = ()> + Send + 'static>(f: F) {
    async_std::task::spawn(f);
}

#[cfg(target_arch = "wasm32")]
fn execute<F: Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}

fn read_file(
    async_rfd: AsyncFileDialog,
    callback: impl FnOnce((String, Vec<u8>)) + 'static + Send,
) {
    let task = async_rfd.pick_file();

    execute(async {
        let file = task.await;

        if let Some(file) = file {
            callback((file.file_name(), file.read().await));
        }
    });
}
