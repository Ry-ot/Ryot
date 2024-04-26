use crate::material::SpriteMaterial;
use bevy_asset::Handle;
use bevy_ecs::prelude::*;
use bevy_hierarchy::prelude::*;
use bevy_render::color::Color;
use bevy_sprite::Anchor;
use bevy_stroked_text::{StrokedText, StrokedTextBundle};
use bevy_transform::prelude::Transform;
use bevy_utils::default;
use glam::Vec3;
use ryot_core::content::ContentId;
use ryot_tiled::prelude::{debug_y_offset, Layer, PositionDebugText};

/// A system that ensures that all entities with an ContentId have a SpriteMaterial mesh bundle.
pub fn debug_sprites(
    mut commands: Commands,
    q_debug: Query<(Entity, &Layer), (With<ContentId>, Without<Handle<SpriteMaterial>>)>,
) {
    q_debug.iter().for_each(|(entity, layer)| {
        commands.entity(entity).with_children(|builder| {
            builder.spawn((
                StrokedTextBundle::new(StrokedText {
                    font_size: 16.,
                    text_anchor: Anchor::BottomRight,
                    ..default()
                })
                .with_transform(
                    Transform::from_translation(Vec3::new(8., debug_y_offset(layer), 1.))
                        .with_scale(Vec3::splat(0.18)),
                ),
                PositionDebugText,
                Layer::Hud(0),
            ));
            builder.spawn((
                StrokedTextBundle::new(StrokedText {
                    text: format!("{}", layer),
                    font_size: 16.,
                    text_anchor: Anchor::BottomLeft,
                    color: Color::from(layer),
                    ..default()
                })
                .with_transform(
                    Transform::from_translation(Vec3::new(8.5, debug_y_offset(layer), 1.))
                        .with_scale(Vec3::splat(0.18)),
                ),
                Layer::Hud(0),
            ));
        });
    });
}
