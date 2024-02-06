use crate::sprites::LoadedSprite;
use crate::{Palette, TilesetCategory};
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_egui::EguiContexts;
use egui::load::SizedTexture;
use egui::{Align, TextureId, Ui};
use ryot::bevy_ryot::AppearanceDescriptor;
use std::ops::Range;

#[derive(Resource, Debug)]
pub struct PaletteState {
    pub min: usize,
    pub max: usize,
    pub width: f32,
    pub grid_size: u32,
    pub tile_padding: f32,
    pub selected_tile: Option<AppearanceDescriptor>,
    pub selected_category: TilesetCategory,
    pub category_sprites: HashMap<u32, u32>,
    pub visible_rows: Range<usize>,
    pub loaded_images: Vec<(LoadedSprite, Handle<Image>, egui::Vec2, egui::Rect)>,
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
            selected_category: TilesetCategory::Raw,
            category_sprites: HashMap::default(),
            visible_rows: Range { start: 0, end: 10 },
            loaded_images: vec![],
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

pub fn get_egui_parameters_for_texture(
    sprite: &LoadedSprite,
    atlas: &TextureAtlas,
) -> Option<(egui::Vec2, egui::Rect)> {
    let rect = atlas.textures.get(sprite.get_sprite_index())?;

    let uv: egui::Rect = egui::Rect::from_min_max(
        egui::pos2(rect.min.x / atlas.size.x, rect.min.y / atlas.size.y),
        egui::pos2(rect.max.x / atlas.size.x, rect.max.y / atlas.size.y),
    );

    let rect_vec2: egui::Vec2 = egui::Vec2::new(rect.max.x - rect.min.x, rect.max.y - rect.min.y);

    Some((rect_vec2, uv))
}

pub fn draw_palette_window(
    palettes: Res<Palette>,
    mut egui_ctx: EguiContexts,
    mut palette_state: ResMut<PaletteState>,
) {
    let categories = palettes.get_categories();
    let binding = palette_state.loaded_images.clone();

    let egui_images = binding
        .iter()
        .map(|(sprite, image, rect, uv)| {
            let tex: TextureId = egui_ctx.add_image(image.clone_weak());
            (
                sprite,
                egui::Image::new(SizedTexture::new(tex, *rect)).uv(*uv),
            )
        })
        .collect::<Vec<_>>();

    egui::Window::new("Palette")
        .max_width(palette_state.width)
        .show(egui_ctx.ctx_mut(), |ui| {
            draw_palette_bottom_panel(ui, &mut palette_state);
            draw_palette_picker(ui, categories, &mut palette_state);
            draw_palette_items(ui, egui_images, palette_state);
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
        .selected_text(palette_state.selected_category.get_label().clone())
        .width(palette_state.width)
        .show_ui(ui, |ui| {
            for key in categories {
                if ui
                    .selectable_value(
                        &mut palette_state.selected_category.get_label(),
                        key.get_label().clone(),
                        key,
                    )
                    .clicked()
                {
                    palette_state.selected_category = *key;
                    palette_state.category_sprites.clear();
                }
            }
        });
}

pub fn draw_palette_items(
    ui: &mut Ui,
    egui_images: Vec<(&LoadedSprite, egui::Image)>,
    mut palette_state: ResMut<PaletteState>,
) {
    let row_padding = 3.;
    let row_height = palette_state.grid_size as f32 + row_padding;

    egui::ScrollArea::vertical()
        .max_height(ui.available_height())
        .show_rows(
            ui,
            row_height,
            palette_state.get_total_rows(palette_state.category_sprites.len()),
            |ui, row_range| {
                ui.set_width(palette_state.width);
                palette_state.visible_rows = row_range.clone();
                egui_images
                    .chunks(palette_state.get_chunk_size())
                    .for_each(|chunk| {
                        ui.horizontal(|ui| {
                            chunk.iter().enumerate().for_each(|(i, (sprite, image))| {
                                let spacing = (palette_state.width
                                    - palette_state.get_tile_size()
                                        * palette_state.get_chunk_size() as f32)
                                    / (palette_state.get_chunk_size() - 1) as f32;
                                let size = palette_state.grid_size as f32;

                                ui.vertical(|ui| {
                                    let tile = image
                                        .clone()
                                        .fit_to_exact_size(egui::Vec2::new(size, size));

                                    let Some(content_id) = palette_state
                                        .category_sprites
                                        .get(&sprite.sprite_id)
                                        .copied()
                                    else {
                                        return;
                                    };

                                    let selected = match &palette_state.selected_tile {
                                        Some(AppearanceDescriptor { id, .. }) => *id == content_id,
                                        _ => false,
                                    };

                                    let ui_button =
                                        ui.add(egui::ImageButton::new(tile).selected(selected));

                                    let ui_button = ui_button
                                        .on_hover_text(format!("{}", content_id))
                                        .on_hover_cursor(egui::CursorIcon::PointingHand);

                                    if ui_button.clicked() {
                                        match palette_state.selected_tile {
                                            Some(AppearanceDescriptor { id, .. })
                                                if id == content_id =>
                                            {
                                                palette_state.selected_tile = None;
                                                debug!("Tile: {:?} deselected", content_id);
                                            }
                                            _ => {
                                                palette_state.selected_tile =
                                                    Some(AppearanceDescriptor::new(
                                                        sprite.group,
                                                        content_id,
                                                        default(),
                                                    ));
                                                debug!("Tile: {:?} selected", content_id);
                                            }
                                        }
                                    }

                                    ui.add_space(row_padding);
                                });

                                let tile_size = sprite.sprite_sheet.get_tile_size(&sprite.config);
                                let ratio = tile_size.x as f32 / tile_size.y as f32;

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
