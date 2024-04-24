use crate::{tile_size, TILE_SIZE};
use bevy_asset::Assets;
use bevy_ecs::change_detection::{Res, ResMut};
use bevy_math::prelude::Rectangle;
use bevy_render::mesh::Mesh;
use bevy_sprite::TextureAtlasLayout;
use bevy_utils::tracing::debug;
use ryot_assets::prelude::{
    AtlasLayoutsAsset, RectMeshes, SpriteLayout, SpriteMeshes, TextureAtlasLayouts,
};
use strum::IntoEnumIterator;

pub fn prepare_sprite_meshes(
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

pub fn prepare_sprite_layouts<C: AtlasLayoutsAsset>(
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
