use crate::prelude::*;
use crate::update::{sprite_material_from_params, ChangingAppearanceFilter};
use bevy_asset::{Assets, Handle};
use bevy_ecs::change_detection::{Res, ResMut};
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Commands, Query, Resource, SystemSet};
use bevy_time::Time;
use bevy_utils::tracing::warn;
use bevy_utils::HashMap;
use derive_more::{Deref, DerefMut};
use ryot_core::content::ContentId;
use ryot_core::prelude::FrameGroup;
use ryot_tiled::prelude::Directional;
use std::time::Duration;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum AnimationSystems {
    Initialize,
    Update,
}

/// An optional component to override animation timers.
#[derive(Component, Default)]
pub struct AnimationDuration(pub Duration);

#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub struct SynchronizedAnimationTimers(HashMap<AnimationKey, AnimationState>);

pub fn initialize_animation_sprite_system(
    mut commands: Commands,
    q_maybe_animated: Query<(Entity, &ContentId, Option<&FrameGroup>), ChangingAppearanceFilter>,
    loaded_appereances: Res<LoadedAppearances>,
    mut synced_timers: ResMut<SynchronizedAnimationTimers>,
) {
    q_maybe_animated
        .iter()
        .for_each(|(entity, object_id, frame_group)| {
            let Some(loaded_appearance) =
                loaded_appereances.get(&(*object_id, frame_group.cloned().unwrap_or_default()))
            else {
                warn!("BUG: Loaded appearance for {:?} not found.", object_id);
                return;
            };
            let Some((ref key, ref descriptor)) = loaded_appearance.animation else {
                commands.entity(entity).remove::<AnimationSprite>();
                return;
            };
            commands
                .entity(entity)
                .insert(AnimationSprite::from_key_and_descriptor(key, descriptor));

            if descriptor.synchronized {
                synced_timers
                    .try_insert(key.clone(), key.default_state())
                    .ok();
            }
        });
}

/// A system that animates the sprites based on the `AnimationSprite` component.
/// It's meant to run every frame to update the animation of the entities.
/// It will only run if the entity has a `TextureAtlas` and an `AnimationSprite` component.
pub fn tick_animation_system(
    time: Res<Time>,
    mut synced_timers: ResMut<SynchronizedAnimationTimers>,
    mut q_sprites: Query<(
        &mut Handle<SpriteMaterial>,
        &mut AnimationSprite,
        Option<&Directional>,
        Option<&SpriteParams>,
        Option<&AnimationDuration>,
    )>,
    mut materials: ResMut<Assets<SpriteMaterial>>,
) {
    let delta = time.delta();
    synced_timers
        .iter_mut()
        .for_each(|(key, state)| state.tick(key, delta));

    q_sprites.iter_mut().for_each(
        |(mut material, mut anim, direction, sprite_params, duration)| {
            if let AnimationSprite::Independent { key, state, .. } = &mut *anim {
                if let Some(duration) = duration {
                    let frame_duration = duration.0 / key.total_phases as u32;
                    if state.timer.duration() != frame_duration {
                        state.timer.set_duration(frame_duration)
                    }
                }
                state.tick(key, delta);
            }

            let (state, descriptor) = match anim.as_ref() {
                AnimationSprite::Independent {
                    state, descriptor, ..
                } => (state, descriptor),
                AnimationSprite::Synchronized { key, descriptor } => {
                    let Some(state) = synced_timers.get(key) else {
                        return;
                    };
                    (state, descriptor)
                }
            };

            if state.just_finished() {
                let direction_index = match direction {
                    Some(dir) => dir.index(),
                    None => 0,
                } * descriptor.layers;
                let Some(sprite) = descriptor
                    .sprites
                    .get(direction_index + state.current_phase * descriptor.skip)
                else {
                    return;
                };
                *material = sprite_material_from_params(sprite_params, &mut materials, sprite);
            }
        },
    );
}
