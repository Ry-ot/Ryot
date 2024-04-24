//! Bevy plugins and utilities for RyOT games.
//!
//! This module is intended to be used as a library dependency for RyOT games.
//! It provides common ways of dealing with OT content, such as loading sprites,
//! configuring the game, and handling asynchronous events.
use crate::prelude::*;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::*;
use bevy_stroked_text::StrokedTextPlugin;

mod game;
pub use game::*;

#[cfg(feature = "lmdb")]
pub mod lmdb;

pub mod drawing;

pub mod sprites;

pub mod position;
pub(crate) mod sprite_animations;

pub use sprite_animations::{toggle_sprite_animation, AnimationDuration};

pub struct RyotLegacySpritePlugin;

impl Plugin for RyotLegacySpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RyotSpritePlugin);

        app.add_optional_plugin(StrokedTextPlugin)
            .init_resource::<sprite_animations::SpriteAnimationEnabled>()
            .init_resource::<sprite_animations::SynchronizedAnimationTimers>()
            .init_resource::<sprites::LoadedAppearances>()
            .add_event::<sprites::LoadAppearanceEvent>()
            .add_systems(
                Update,
                (
                    #[cfg(feature = "debug")]
                    debug_sprite_position,
                    sprites::load_from_entities_system.in_set(SpriteSystems::Load),
                    sprites::process_load_events_system
                        .pipe(sprites::load_sprite_system)
                        .pipe(sprites::store_loaded_appearances_system)
                        .run_if(on_event::<sprites::LoadAppearanceEvent>())
                        .in_set(SpriteSystems::Load),
                    (
                        #[cfg(feature = "debug")]
                        sprites::debug_sprites,
                        sprites::initialize_elevation,
                    )
                        .in_set(SpriteSystems::Initialize),
                    sprites::update_sprite_system.in_set(SpriteSystems::Update),
                    sprite_animations::initialize_animation_sprite_system
                        .in_set(AnimationSystems::Initialize),
                    sprite_animations::tick_animation_system
                        .run_if(resource_exists_and_equals(
                            sprite_animations::SpriteAnimationEnabled(true),
                        ))
                        .in_set(AnimationSystems::Update),
                )
                    .chain()
                    .run_if(in_state(RyotContentState::Ready)),
            )
            .add_systems(
                PostUpdate,
                (
                    update_sprite_position,
                    (move_sprites_with_animation, finish_position_animation).chain(),
                ),
            );
    }
}
