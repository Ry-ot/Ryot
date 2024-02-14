use crate::OptionalPlugin;
use bevy::asset::Handle;
use bevy::prelude::*;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy_egui::{EguiContexts, EguiPlugin};
use egui::emath::{vec2, Vec2};
use image::Rgba;

const DEFAULT_MINIMAP_ZOOM: f32 = 1.0;
const DEFAULT_MINIMAP_TITLE: &str = "Minimap";
const DEFAULT_TEXTURE_DIMENSION: u32 = 2 * 2048;
const DEFAULT_MAP_DIMENSION: UVec2 = UVec2::new(65536, 65536);
const DEFAULT_MINIMAP_IMAGE_SIZE: Vec2 = vec2(256.0, 256.0);
const DEFAULT_MINIMAP_WINDOW_SIZE: Vec2 = vec2(265.0, 265.0);

pub struct MinimapPlugin;

impl Plugin for MinimapPlugin {
    fn build(&self, app: &mut App) {
        app.add_optional_plugin(EguiPlugin)
            .init_resource::<Minimap>()
            .add_systems(Startup, setup_default_texture)
            .add_systems(Update, draw_minimap_window);
    }
}

#[derive(Resource, Debug)]
pub struct Minimap {
    pub zoom: f32,
    pub title: String,
    pub image_size: Vec2,
    pub window_size: Vec2,
    pub map_size: UVec2,
    pub texture_size: Extent3d,
    pub image_handler: Option<Handle<Image>>,
}

impl Default for Minimap {
    fn default() -> Self {
        Self {
            zoom: DEFAULT_MINIMAP_ZOOM,
            title: DEFAULT_MINIMAP_TITLE.to_string(),
            image_size: DEFAULT_MINIMAP_IMAGE_SIZE,
            window_size: DEFAULT_MINIMAP_WINDOW_SIZE,
            map_size: DEFAULT_MAP_DIMENSION,
            texture_size: Extent3d {
                width: DEFAULT_TEXTURE_DIMENSION,
                height: DEFAULT_TEXTURE_DIMENSION,
                depth_or_array_layers: 1,
            },
            image_handler: None,
        }
    }
}

impl Minimap {
    pub fn get_image_size(&self) -> Vec2 {
        self.image_size * self.zoom
    }

    pub fn initialize_texture(&mut self, mut textures: ResMut<Assets<Image>>) {
        let mut texture = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size: self.texture_size,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba8UnormSrgb,
                usage: TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_DST
                    | TextureUsages::RENDER_ATTACHMENT,
                mip_level_count: 1,
                sample_count: 1,
                view_formats: &[],
            },
            ..Default::default()
        };

        // Fill the texture with 4 RGBA pixel per position
        texture.data = vec![0; (self.texture_size.width * self.texture_size.height * 4) as usize];

        // Initialize the array with RGBA values (0,0,0,255):
        //      3 pixels with 0 = black
        //      1 pixel with 255 = opaque
        texture
            .data
            .iter_mut()
            .enumerate()
            .for_each(|(index, pixel)| {
                if index % 4 != 3 {
                    *pixel = 0;
                } else {
                    *pixel = 255;
                }
            });

        self.image_handler = Some(textures.add(texture));
    }

    pub fn update_texture(&mut self, tiles: Vec<UVec2>, images: &mut Assets<Image>) {
        if let Some(handler) = &self.image_handler {
            if let Some(texture) = images.get_mut(handler) {
                let data = &mut texture.data;

                for UVec2 { x, y } in tiles {
                    // Calculate the position on the texture
                    let x = x / (self.map_size.x / self.texture_size.width);
                    let y = y / (self.map_size.y / self.texture_size.height);

                    let pixel_index = (y * self.texture_size.width + x) as usize * 4;
                    if pixel_index < data.len() {
                        // Example: Setting the pixel to white
                        data[pixel_index] = 255; // R
                        data[pixel_index + 1] = 255; // G
                        data[pixel_index + 2] = 255; // B
                        data[pixel_index + 3] = 255; // A
                    }
                }
            }
        }
    }
}

pub fn draw_minimap_window(mut contexts: EguiContexts, minimap: ResMut<Minimap>) {
    let image = contexts.add_image(minimap.image_handler.clone().unwrap());

    egui::Window::new(&minimap.title)
        .fixed_size(minimap.window_size)
        .show(contexts.ctx_mut(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.image(egui::load::SizedTexture::new(
                    image,
                    minimap.get_image_size(),
                ));
            });
        });
}

pub fn setup_default_texture(mut minimap: ResMut<Minimap>, textures: ResMut<Assets<Image>>) {
    minimap.initialize_texture(textures);
}

pub fn color_from_8bits(color: u32) -> Rgba<u8> {
    color_from_8bits_with_brightness(color, 1.0)
}

pub fn color_from_8bits_with_brightness(color: u32, brightness: f32) -> Rgba<u8> {
    if color >= 216 || color == 0 {
        return Rgba([0, 0, 0, 0]);
    }

    let r = ((color / 36 % 6 * 51) as f32 * brightness) as u8;
    let g = ((color / 6 % 6 * 51) as f32 * brightness) as u8;
    let b = ((color % 6 * 51) as f32 * brightness) as u8;

    Rgba([r, g, b, 255])
}
