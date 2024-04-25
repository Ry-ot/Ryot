use crate::prelude::{Layer, TilePosition};
use bevy_ecs::change_detection::{Res, ResMut};
use bevy_ecs::prelude::Commands;
use bevy_render::color::Color;
use bevy_render::mesh::{Indices, Mesh, PrimitiveTopology};
use bevy_render::render_asset::RenderAssetUsages;
use bevy_transform::components::Transform;
use bevy_utils::default;
use glam::Vec2;
use ryot_core::prelude::*;

pub static GRID_LAYER: Layer = Layer::Hud(0);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::prelude::Component))]
pub struct GridView;

/// A system to spawn a grid of lines to represent the tiles in the game using a custom color.
#[cfg(feature = "bevy")]
pub fn spawn_grid(
    color: Color,
) -> impl FnMut(
    Commands,
    Res<TextureAtlasLayouts>,
    ResMut<bevy_asset::Assets<Mesh>>,
    ResMut<bevy_asset::Assets<bevy_sprite::ColorMaterial>>,
) {
    move |mut commands: Commands,
          atlas_layouts: Res<TextureAtlasLayouts>,
          mut meshes: ResMut<bevy_asset::Assets<Mesh>>,
          mut materials: ResMut<bevy_asset::Assets<bevy_sprite::ColorMaterial>>| {
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

        let mesh_handle: bevy_asset::Handle<Mesh> = meshes.add(mesh);

        commands.spawn((
            GridView,
            bevy_sprite::MaterialMesh2dBundle {
                mesh: mesh_handle.into(),
                transform: Transform::from_translation(Vec2::ZERO.extend(GRID_LAYER.z())),
                material: materials.add(bevy_sprite::ColorMaterial::default()),
                ..default()
            },
        ));
    }
}
