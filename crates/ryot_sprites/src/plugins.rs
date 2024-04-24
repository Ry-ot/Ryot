use crate::atlas::TextureAtlasLayouts;
use crate::material::SpriteMaterial;
use crate::meshes::{RectMeshes, SpriteMeshes};
use bevy_app::*;
use bevy_asset::embedded_asset;
use bevy_sprite::Material2dPlugin;

pub struct RyotSpritePlugin;

impl Plugin for RyotSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<SpriteMaterial>::default())
            .init_resource::<RectMeshes>()
            .init_resource::<SpriteMeshes>()
            .init_resource::<TextureAtlasLayouts>();

        embedded_asset!(app, "material/shaders/sprite.wgsl");
    }
}
