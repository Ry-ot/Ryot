use bevy_app::App;
use bevy_asset::{embedded_asset, Asset, Handle};
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Commands, Query, With, Without};
use bevy_reflect::TypePath;
use bevy_render::color::Color;
use bevy_render::prelude::Image;
use bevy_render::render_resource::{AsBindGroup, ShaderRef};
use bevy_sprite::{Material2d, MaterialMesh2dBundle};
use glam::Vec2;
use ryot_core::prelude::{ContentId, SpriteLayout};

pub mod meshes;
pub mod params;

#[derive(AsBindGroup, TypePath, Asset, Debug, Clone, Default, PartialEq)]
pub struct SpriteMaterial {
    #[uniform(0)]
    pub index: u32,
    #[uniform(0)]
    pub counts: Vec2,
    #[uniform(0)]
    pub outline_thickness: f32,
    #[uniform(0)]
    pub outline_color: Color,
    #[uniform(0)]
    pub tint: Color,
    #[uniform(0)]
    pub alpha: f32,
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
}

impl Material2d for SpriteMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://ryot_sprites/material/shaders/sprite.wgsl".into()
    }
}

pub fn embed_sprite_assets(app: &mut App) {
    embedded_asset!(app, "shaders/sprite.wgsl");
}

/// A system that ensures that all entities with an and Id component
/// have a SpriteMaterial mesh bundle.
pub fn initialize_sprite_material(
    mut commands: Commands,
    query: Query<Entity, (With<ContentId>, Without<Handle<SpriteMaterial>>)>,
) {
    query.iter().for_each(|entity| {
        commands.entity(entity).insert((
            MaterialMesh2dBundle::<SpriteMaterial>::default(),
            SpriteLayout::default(),
        ));
    });
}
