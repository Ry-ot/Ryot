use std::marker::PhantomData;

use crate::{
    draw_palette_bottom_panel, draw_palette_items, draw_palette_picker, toggle_grid, Cursor,
    InputType, OptionalPlugin, Palette, PaletteState, ToolMode,
};

#[cfg(not(target_arch = "wasm32"))]
use crate::{ExportMap, LoadMap};
#[cfg(not(target_arch = "wasm32"))]
use ryot::bevy_ryot::{AsyncEventApp, EventSender};

use bevy::{app::AppExit, prelude::*, render::camera::Viewport, winit::WinitWindows};
use bevy_egui::{EguiContext, EguiContexts, EguiPlugin, EguiUserTextures};
use egui::{load::SizedTexture, Slider, TextureId};
use egui_dock::{DockArea, DockState, NodeIndex};
use ryot::{
    bevy_ryot::{
        drawing::{Brushes, DrawingBundle},
        ContentAssets, GridView, InternalContentState,
    },
    include_svg,
};

const DELETE_ICON: egui::ImageSource = include_svg!(
    r##"
    <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill="#fff6c2" viewBox="0 0 256 256"><path d="M216,48H176V40a24,24,0,0,0-24-24H104A24,24,0,0,0,80,40v8H40a8,8,0,0,0,0,16h8V208a16,16,0,0,0,16,16H192a16,16,0,0,0,16-16V64h8a8,8,0,0,0,0-16ZM112,168a8,8,0,0,1-16,0V104a8,8,0,0,1,16,0Zm48,0a8,8,0,0,1-16,0V104a8,8,0,0,1,16,0Zm0-120H96V40a8,8,0,0,1,8-8h48a8,8,0,0,1,8,8Z"></path></svg>
    "##
);

const GRID_ICON: egui::ImageSource = include_svg!(
    r##"
    <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill="#fff6c2" viewBox="0 0 256 256"><path d="M200,40H56A16,16,0,0,0,40,56V200a16,16,0,0,0,16,16H200a16,16,0,0,0,16-16V56A16,16,0,0,0,200,40Zm0,80H136V56h64ZM120,56v64H56V56ZM56,136h64v64H56Zm144,64H136V136h64v64Z"></path></svg>
    "##
);

pub struct UiPlugin<C: ContentAssets>(PhantomData<C>);

impl<C: ContentAssets> Default for UiPlugin<C> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<C: ContentAssets> Plugin for UiPlugin<C> {
    fn build(&self, app: &mut App) {
        app.add_optional_plugin(EguiPlugin)
            .init_resource::<UiState>()
            .add_systems(First, check_egui_usage)
            .add_systems(OnEnter(InternalContentState::Ready), add_editor)
            .add_systems(
                Update,
                (
                    ui_menu_system::<C>,
                    ui_dock_system,
                    resize_camera_viewport_system.map(drop),
                )
                    .chain()
                    .run_if(in_state(InternalContentState::Ready)),
            );

        #[cfg(not(target_arch = "wasm32"))]
        app.add_event::<ExportMap>().add_async_event::<LoadMap>();
    }
}

#[allow(clippy::too_many_arguments)]
fn ui_menu_system<C: ContentAssets>(
    content_assets: Res<C>,
    brushes: Res<Brushes<DrawingBundle>>,
    q_grid: Query<&mut Visibility, With<GridView>>,
    mut cursor_query: Query<&mut Cursor>,
    mut contexts: Query<&mut EguiContext>,
    mut exit: EventWriter<AppExit>,
    #[cfg(not(target_arch = "wasm32"))] mut map_export_sender: EventWriter<ExportMap>,
    #[cfg(not(target_arch = "wasm32"))] load_map_sender: Res<EventSender<LoadMap>>,
    _windows: NonSend<WinitWindows>,
) {
    let Ok(mut cursor) = cursor_query.get_single_mut() else {
        return;
    };
    let mut egui_ctx = contexts.single_mut();
    egui::TopBottomPanel::top("top_panel").show(egui_ctx.get_mut(), |ui| {
        egui::menu::bar(ui, |ui| {
            ui.scope(|ui| {
                let mut style = (*ui.ctx().style()).clone();

                // Modify the style for your specific widget
                style.visuals.widgets.inactive.bg_fill = egui::Color32::GRAY;
                style.visuals.widgets.active.bg_fill = egui::Color32::GRAY;
                style.visuals.widgets.hovered.bg_fill = egui::Color32::GRAY;

                // Temporarily apply the style
                ui.set_style(style);

                let is_content_loaded = content_assets.sprite_sheet_data_set().is_some();

                egui::menu::menu_button(ui, "File", |ui| {
                    if ui
                        .add_enabled(is_content_loaded, egui::Button::new("🗁 Open"))
                        .clicked()
                    {
                        // #[cfg(target_arch = "wasm32")]
                        // read_file(
                        //     rfd::AsyncFileDialog::new().add_filter(".mdb", &["mdb"]),
                        //     |(file_name, content)| {
                        //         debug!("Loading map from file: {:?}", file_name);
                        //         debug!("Current dir: {:?}", std::env::current_dir());
                        //     },
                        // );

                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let path = rfd::FileDialog::new()
                                .add_filter(".mdb", &["mdb"])
                                .pick_file();

                            if let Some(path) = path {
                                if let Err(e) = load_map_sender.send(LoadMap(path)) {
                                    warn!("Failed to send load map event: {}", e);
                                }
                            }
                        }
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    if ui
                        .add_enabled(is_content_loaded, egui::Button::new("💾 Save"))
                        .clicked()
                    {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter(".mdb", &["mdb"])
                            .save_file()
                        {
                            debug!("Saving map to file: {:?}", path);
                            map_export_sender.send(ExportMap(path));
                        }
                    }

                    ui.separator();

                    if ui.button("Exit").clicked() {
                        exit.send(AppExit);
                    }
                });

                egui::menu::menu_button(ui, "About", |ui| ui.button("Compass v0.1.0"));
            });
        });

        ui.horizontal_centered(|ui| {
            ui.set_max_height(24.);
            ui.horizontal(|ui| {
                let mut style = (*ui.ctx().style()).clone();
                style.visuals.interact_cursor = Some(egui::CursorIcon::PointingHand);

                let grid_button = egui::ImageButton::new(GRID_ICON).selected(
                    q_grid
                        .get_single()
                        .map(|v| !matches!(v, Visibility::Hidden))
                        .unwrap_or(false),
                );
                let grid_button = ui.add_sized(egui::Vec2::new(24., 24.), grid_button);
                if grid_button.on_hover_text("Toggle Grid").clicked() {
                    toggle_grid(q_grid);
                }

                ui.separator();

                for (index, brush) in brushes.iter().enumerate() {
                    let is_selected = cursor.drawing_state.brush_index == index;
                    let button = egui::ImageButton::new(brush.icon()).selected(is_selected);
                    let button = ui.add_sized(egui::Vec2::new(24., 24.), button);
                    if button.on_hover_text(brush.name()).clicked() {
                        cursor.drawing_state.brush_index = index;
                    }
                }

                ui.separator();

                ui.scope(|ui| {
                    ui.style_mut().visuals.selection.bg_fill =
                        egui::Color32::RED.gamma_multiply(0.5);
                    let delete_button = egui::ImageButton::new(DELETE_ICON)
                        .selected(cursor.drawing_state.tool_mode == ToolMode::Erase);
                    let delete_button = ui.add_sized(egui::Vec2::new(24., 24.), delete_button);
                    if delete_button.on_hover_text("Delete").clicked() {
                        cursor.drawing_state.tool_mode =
                            if cursor.drawing_state.tool_mode == ToolMode::Erase {
                                ToolMode::Draw
                            } else {
                                ToolMode::Erase
                            };
                    }
                });
            });

            ui.separator();

            let current_brush_index = &mut cursor.drawing_state.brush_index;
            ui.add_sized(
                egui::Vec2::new(18., 18.),
                egui::Image::new(brushes[*current_brush_index].icon()).tint(egui::Color32::GRAY),
            );
            if let InputType::SingleClick(size) = &mut cursor.drawing_state.input_type {
                ui.add(Slider::new(size, 0..=20));
            }
        });

        ui.add_space(4.);
    });
}

fn ui_dock_system(
    mut contexts: Query<&mut EguiContext>,
    mut ui_state: ResMut<UiState>,
    egui_user_textures: ResMut<EguiUserTextures>,
    palettes: Res<Palette>,
    palette_state: ResMut<PaletteState>,
) {
    let mut ctx = contexts.single_mut();
    ui_state.ui(ctx.get_mut(), egui_user_textures, palettes, palette_state);
}

fn resize_camera_viewport_system(
    contexts: EguiContexts,
    mut camera_query: Query<&mut Camera>,
    ui_state: Res<UiState>,
) -> Option<()> {
    let ctx = contexts.ctx();
    let scale = ctx.pixels_per_point();
    for mut camera in camera_query.iter_mut() {
        for tab in ui_state.state.iter_all_tabs() {
            if let ((sfc_idx, node_idx), EguiWindow::Editor(_)) = tab {
                let rect = ui_state.state[sfc_idx][node_idx].rect()?;
                camera.viewport = Some(Viewport {
                    physical_position: (scale * Vec2::new(rect.min.x, rect.min.y)).as_uvec2(),
                    physical_size: (scale * Vec2::new(rect.width(), rect.height())).as_uvec2(),
                    ..default()
                });
                continue;
            }
        }
    }

    Some(())
}

fn add_editor(mut ui_state: ResMut<UiState>) {
    ui_state
        .state
        .main_surface_mut()
        .push_to_focused_leaf(EguiWindow::Editor("editor".to_string()));
}

#[derive(Component, Default)]
pub struct Buffer(pub Handle<Image>);

#[derive(Debug)]
enum EguiWindow {
    Editor(String),
    Palette,
}

/// The GUIState resource is used to keep track of whether the GUI is being used.
/// This is useful for systems that should only run when the GUI is/is not being used.
/// For example, drawing systems should only run when the GUI is not being used.
#[derive(Resource)]
pub struct UiState {
    pub is_being_used: bool,
    state: DockState<EguiWindow>,
}

impl Default for UiState {
    fn default() -> Self {
        Self::new()
    }
}

impl UiState {
    pub fn new() -> Self {
        let mut state = DockState::new(vec![]);
        let tree = state.main_surface_mut();
        let [editor, _palette] = tree.split_left(NodeIndex::root(), 0.3, vec![EguiWindow::Palette]);
        tree.set_focused_node(editor);

        Self {
            state,
            is_being_used: false,
        }
    }

    fn ui(
        &mut self,
        ctx: &mut egui::Context,
        egui_user_textures: ResMut<EguiUserTextures>,
        palettes: Res<Palette>,
        palette_state: ResMut<PaletteState>,
    ) {
        let mut tab_viewer = TabViewer {
            egui_user_textures,
            palettes,
            palette_state,
        };

        egui::CentralPanel::default()
            .frame(
                egui::Frame::central_panel(&ctx.style())
                    .inner_margin(0.)
                    .fill(egui::Color32::from_black_alpha(0)),
            )
            .show(ctx, |ui| {
                let style = egui_dock::Style::from_egui(ctx.style().as_ref());
                DockArea::new(&mut self.state)
                    .style(style)
                    .show_inside(ui, &mut tab_viewer)
            });
    }
}

struct TabViewer<'a> {
    egui_user_textures: ResMut<'a, EguiUserTextures>,
    palettes: Res<'a, Palette>,
    palette_state: ResMut<'a, PaletteState>,
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = EguiWindow;

    fn ui(&mut self, ui: &mut egui_dock::egui::Ui, tab: &mut Self::Tab) {
        match tab {
            EguiWindow::Editor(_) => {}
            EguiWindow::Palette => {
                let categories = self.palettes.get_categories();
                let binding = self.palette_state.loaded_images.clone();
                ui.set_min_width(self.palette_state.min_width());
                let viewport_size = ui.clip_rect();
                self.palette_state.width = viewport_size.width() - 20.0;

                let egui_images = binding
                    .iter()
                    .map(|(sprite, image, rect, uv)| {
                        let tex: TextureId = self.egui_user_textures.add_image(image.clone_weak());
                        (
                            sprite,
                            egui::Image::new(SizedTexture::new(tex, *rect)).uv(*uv),
                        )
                    })
                    .collect::<Vec<_>>();

                draw_palette_bottom_panel(ui, &mut self.palette_state);
                draw_palette_picker(ui, categories, &mut self.palette_state);
                draw_palette_items(ui, egui_images, &mut self.palette_state);
            }
        }
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui_dock::egui::WidgetText {
        if let EguiWindow::Editor(title) = tab {
            title.to_string().into()
        } else {
            format!("{tab:?}").into()
        }
    }

    fn allowed_in_windows(&self, tab: &mut Self::Tab) -> bool {
        !matches!(tab, EguiWindow::Editor(_))
    }

    fn clear_background(&self, tab: &Self::Tab) -> bool {
        !matches!(tab, EguiWindow::Editor(_))
    }

    fn scroll_bars(&self, tab: &Self::Tab) -> [bool; 2] {
        if matches!(tab, EguiWindow::Editor(_)) {
            [false, false]
        } else {
            [true, true]
        }
    }
}

/// This condition checks if the GUI is being used and can be used with run_if.
/// ```rust
/// use bevy::prelude::*;
/// use bevy_egui::EguiContext;
/// use ryot_compass::gui_is_in_use;
///
/// fn gui_is_active_system() {
///     info!("GUI is active");
/// }
///
/// fn main() {
///   App::new().add_systems(Update, gui_is_active_system.run_if(gui_is_in_use()));
/// }
/// ```
pub fn gui_is_in_use() -> impl FnMut(Res<UiState>) -> bool + Clone {
    move |gui_state| gui_state.is_being_used
}

/// This condition checks if the GUI is not being used and can be used with run_if.
/// ```rust
/// use bevy::prelude::*;
/// use bevy_egui::EguiContext;
/// use ryot_compass::gui_is_not_in_use;
///
/// fn gui_is_not_active_system() {
///     info!("GUI is not active");
/// }
///
/// fn main() {
///   App::new().add_systems(Update, gui_is_not_active_system.run_if(gui_is_not_in_use()));
/// }
/// ```
pub fn gui_is_not_in_use() -> impl FnMut(Res<UiState>) -> bool + Clone {
    move |gui_state| !gui_state.is_being_used
}

fn is_cursor_over_editor(egui: &Query<&EguiContext>, gui_state: &ResMut<UiState>) -> bool {
    let egui = egui.single();
    if let Some(cursor_pos) = egui.get().pointer_hover_pos() {
        gui_state
            .state
            .iter_all_tabs()
            .filter(|(_, window)| matches!(window, EguiWindow::Editor(_)))
            .any(|((sfc_idx, node_idx), _)| {
                if let Some(rect) = gui_state.state[sfc_idx][node_idx].rect() {
                    if rect.contains(cursor_pos) {
                        return true;
                    }
                }
                false
            })
    } else {
        false
    }
}

/// This system updates the GUIState resource to indicate whether EGUI is being used or not.
pub fn check_egui_usage(egui: Query<&EguiContext>, mut gui_state: ResMut<UiState>) {
    if is_cursor_over_editor(&egui, &gui_state) {
        gui_state.is_being_used = false;
        return;
    }
    let egui = egui.single();
    gui_state.is_being_used = egui.get().wants_pointer_input() || egui.get().wants_keyboard_input();
}
