//! Sprite loading and drawing.
use crate::animation::descriptor::AnimationSprite;
use crate::prelude::*;
use bevy_asset::*;
use bevy_ecs::prelude::*;
use bevy_sprite::Mesh2dHandle;
use bevy_utils::tracing::warn;
use ryot_core::prelude::*;
use ryot_tiled::prelude::*;

pub(crate) type ChangingAppearanceFilter = Or<(
    Changed<ContentId>,
    Changed<FrameGroup>,
    Changed<Directional>,
    (Without<AnimationSprite>, Changed<SpriteParams>),
)>;

pub fn update_sprite_system(
    mut q_updated: Query<
        (
            &ContentId,
            Option<&FrameGroup>,
            Option<&Directional>,
            Option<&SpriteParams>,
            &mut SpriteLayout,
            &mut Mesh2dHandle,
            &mut Handle<SpriteMaterial>,
        ),
        ChangingAppearanceFilter,
    >,
    mut materials: ResMut<Assets<SpriteMaterial>>,
    loaded_appereances: Res<LoadedAppearances>,
) {
    q_updated.iter_mut().for_each(
        |(object_id, frame_group, direction, sprite_params, mut layout, mut mesh, mut material)| {
            if object_id.is_none() {
                return;
            }
            let Some(loaded_appearance) =
                loaded_appereances.get(&(*object_id, frame_group.cloned().unwrap_or_default()))
            else {
                warn!("BUG: Loaded appearance for {:?} not found.", object_id);
                return;
            };

            let direction_index = match direction {
                Some(dir) => dir.index(),
                None => 0,
            } * loaded_appearance.layers as usize;

            let Some(sprite) = loaded_appearance.sprites.get(direction_index) else {
                warn!(
                    "Sprite for appearance {:?} not found for direction {:?}",
                    object_id, direction
                );
                return;
            };
            *layout = sprite.sprite_sheet.layout;
            *mesh = Mesh2dHandle(sprite.mesh.clone());
            *material = sprite_material_from_params(sprite_params, &mut materials, sprite);
        },
    );
}

pub(crate) fn sprite_material_from_params(
    sprite_params: Option<&SpriteParams>,
    materials: &mut ResMut<'_, Assets<SpriteMaterial>>,
    sprite: &LoadedSprite,
) -> Handle<SpriteMaterial> {
    sprite_params
        .map(|params| {
            if params.has_any() {
                materials
                    .get(sprite.material.id())
                    .map(|base| params.to_material(base.clone()))
                    .map(|material| materials.add(material))
                    .unwrap_or_else(|| sprite.material.clone())
            } else {
                sprite.material.clone()
            }
        })
        .unwrap_or_else(|| sprite.material.clone())
}
