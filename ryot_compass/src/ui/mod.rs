use crate::TilesetCategory;
use bevy::prelude::{debug, ResMut, Resource};
use bevy_egui::EguiContexts;
use egui::{Align, Ui};
use ryot::*;
use std::ops::Range;

#[derive(Resource, Debug)]
pub struct PaletteState {
    pub min: usize,
    pub max: usize,
    pub width: f32,
    pub grid_size: u32,
    pub tile_padding: f32,
    pub selected_tile: Option<u32>,
    pub category: TilesetCategory,
    pub visible_rows: Range<usize>,
}

impl Default for PaletteState {
    fn default() -> Self {
        Self {
            min: 4,
            max: 9,
            width: 424.,
            grid_size: 64,
            tile_padding: 15.,
            selected_tile: None,
            category: TilesetCategory::Terrains,
            visible_rows: Range { start: 0, end: 10 },
        }
    }
}

impl PaletteState {
    pub fn get_chunk_size(&self) -> usize {
        ((self.width / self.get_tile_size()) as usize).clamp(self.min, self.max)
    }

    pub fn get_total_rows(&self, items_count: usize) -> usize {
        items_count / self.get_chunk_size()
    }

    pub fn begin(&self) -> usize {
        self.visible_rows.start * self.get_chunk_size()
    }

    pub fn end(&self) -> usize {
        self.visible_rows.end * self.get_chunk_size()
    }

    pub fn get_tile_size(&self) -> f32 {
        self.grid_size as f32 + self.tile_padding
    }
}

pub fn draw_palette_window(
    items_count: usize,
    categories: Vec<&TilesetCategory>,
    egui_images: Vec<(u32, SheetGrid, egui::Image)>,
    mut egui_ctx: EguiContexts,
    mut palette_state: ResMut<PaletteState>,
) {
    egui::Window::new("Palette")
        .max_width(palette_state.width)
        .show(egui_ctx.ctx_mut(), |ui| {
            draw_palette_bottom_panel(ui, &mut palette_state);
            draw_palette_picker(ui, categories, &mut palette_state);
            draw_palette_items(ui, items_count, egui_images, palette_state);
        });
}

pub fn draw_palette_bottom_panel(ui: &mut Ui, palette_state: &mut ResMut<PaletteState>) {
    egui::TopBottomPanel::bottom("bottom_panel").show_inside(ui, |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            ui.set_width(palette_state.width);
            ui.add_space(5.0); // Add some space from the top border
            ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                let mut slider_value = palette_state.grid_size as f32;

                ui.add(
                    egui::DragValue::new(&mut slider_value)
                        .clamp_range(32..=80)
                        .custom_formatter(|n, _| format!("{n}x{n}"))
                        .custom_parser(|s| {
                            let parts: Vec<&str> = s.split('x').collect();
                            let n = parts.first()?;
                            n.parse::<f64>().ok()
                        }),
                );

                palette_state.grid_size = get_grid_size_from_slider(slider_value);
            });
        });
    });
}

fn get_grid_size_from_slider(slider_value: f32) -> u32 {
    let snap_values = [32, 48, 64, 80];
    let mut nearest_value = snap_values[0];
    let mut smallest_diff = (slider_value - nearest_value as f32).abs();

    for &snap_value in &snap_values[1..] {
        let diff = (slider_value - snap_value as f32).abs();
        if diff < smallest_diff {
            smallest_diff = diff;
            nearest_value = snap_value;
        }
    }

    nearest_value
}

pub fn draw_palette_picker(
    ui: &mut Ui,
    categories: Vec<&TilesetCategory>,
    palette_state: &mut ResMut<PaletteState>,
) {
    egui::ComboBox::from_id_source("palette")
        .selected_text(palette_state.category.get_label().clone())
        .width(palette_state.width)
        .show_ui(ui, |ui| {
            for key in categories {
                if ui
                    .selectable_value(
                        &mut palette_state.category.get_label(),
                        key.get_label().clone(),
                        key,
                    )
                    .clicked()
                {
                    palette_state.category = *key;
                }
            }
        });
}

pub fn draw_palette_items(
    ui: &mut Ui,
    items_count: usize,
    egui_images: Vec<(u32, SheetGrid, egui::Image)>,
    mut palette_state: ResMut<PaletteState>,
) {
    let row_padding = 3.;
    let row_height = palette_state.grid_size as f32 + row_padding;

    egui::ScrollArea::vertical()
        .max_height(ui.available_height())
        .show_rows(
            ui,
            row_height,
            palette_state.get_total_rows(items_count),
            |ui, row_range| {
                ui.set_width(palette_state.width);
                palette_state.visible_rows = row_range.clone();
                egui_images
                    .chunks(palette_state.get_chunk_size())
                    .for_each(|chunk| {
                        ui.horizontal(|ui| {
                            chunk
                                .iter()
                                .enumerate()
                                .for_each(|(i, (index, grid, image))| {
                                    let spacing = (palette_state.width
                                        - palette_state.get_tile_size()
                                            * palette_state.get_chunk_size() as f32)
                                        / (palette_state.get_chunk_size() - 1) as f32;
                                    let size = palette_state.grid_size as f32;

                                    ui.vertical(|ui| {
                                        let tile = image
                                            .clone()
                                            .fit_to_exact_size(egui::Vec2::new(size, size));

                                        let selected = match palette_state.selected_tile {
                                            Some(selected_index) => selected_index == *index,
                                            _ => false,
                                        };

                                        let ui_button =
                                            ui.add(egui::ImageButton::new(tile).selected(selected));

                                        let ui_button = ui_button
                                            .on_hover_text(format!("{}", index))
                                            .on_hover_cursor(egui::CursorIcon::PointingHand);

                                        if ui_button.clicked() {
                                            palette_state.selected_tile = Some(*index);
                                            debug!("Tile: {:?} selected", index);
                                        }

                                        ui.add_space(row_padding);
                                    });

                                    let ratio = grid.tile_size.x as f32 / grid.tile_size.y as f32;

                                    if i == palette_state.get_chunk_size() - 1 {
                                        return;
                                    }

                                    ui.add_space(spacing);

                                    if ratio > 1.0 && i < palette_state.get_chunk_size() - 1 {
                                        ui.add_space(size / 2.);
                                    }
                                });
                        });
                    });
            },
        );
}
