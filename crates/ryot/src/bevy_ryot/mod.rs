//! Bevy plugins and utilities for RyOT games.
//!
//! This module is intended to be used as a library dependency for RyOT games.
//! It provides common ways of dealing with OT content, such as loading sprites,
//! configuring the game, and handling asynchronous events.
use crate::prelude::*;
use bevy::prelude::*;
use bevy_stroked_text::StrokedTextPlugin;
pub use ryot_sprites::prelude;

mod game;
pub use game::*;

pub struct RyotLegacySpritePlugin;

impl Plugin for RyotLegacySpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RyotSpritePlugin);

        app.add_optional_plugin(StrokedTextPlugin)
            .init_resource::<SpriteAnimationEnabled>()
            .init_resource::<SynchronizedAnimationTimers>()
            .init_resource::<LoadedAppearances>()
            .add_event::<LoadAppearanceEvent>()
            .add_systems(
                Update,
                (
                    #[cfg(feature = "debug")]
                    debug_sprite_position,
                    load_from_entities_system.in_set(SpriteSystems::Load),
                    process_load_events_system
                        .pipe(load_sprite_system)
                        .pipe(store_loaded_appearances_system)
                        .run_if(on_event::<LoadAppearanceEvent>())
                        .in_set(SpriteSystems::Load),
                    // #[cfg(feature = "debug")]
                    // debug_sprites.in_set(SpriteSystems::Initialize),
                    update_sprite_system
                        .in_set(SpriteSystems::Update)
                        .after(SpriteSystems::Initialize),
                    initialize_animation_sprite_system.in_set(AnimationSystems::Initialize),
                    tick_animation_system
                        .run_if(resource_exists_and_equals(SpriteAnimationEnabled(true)))
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
