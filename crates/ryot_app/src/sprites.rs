use bevy_app::{App, Plugin, PostUpdate, Update};
use bevy_ecs::prelude::*;
use bevy_render::view::{check_visibility, VisibilitySystems};
use bevy_sprite::Material2dPlugin;
use bevy_stroked_text::StrokedTextPlugin;
use ryot_content::prelude::*;
use ryot_core::plugins::OptionalPlugin;
use ryot_sprites::prelude::*;
use ryot_tiled::prelude::*;

pub struct RyotDrawingPlugin;

impl Plugin for RyotDrawingPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Layer>()
            .init_resource::<MapTiles<Entity>>()
            .add_systems(
                PostUpdate,
                apply_detail_level_to_visibility
                    .in_set(VisibilitySystems::CheckVisibility)
                    .after(check_visibility)
                    .run_if(in_state(RyotContentState::Ready)),
            )
            .add_systems(
                PostUpdate,
                (apply_update, apply_deletion)
                    .in_set(DrawingSystems::Apply)
                    .after(VisibilitySystems::VisibilityPropagate),
            )
            .add_systems(
                PostUpdate,
                (persist_update, persist_deletion)
                    .in_set(DrawingSystems::Persist)
                    .after(DrawingSystems::Apply),
            );
    }
}

pub struct RyotSpritePlugin;

impl Plugin for RyotSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_optional_plugin(StrokedTextPlugin)
            .init_resource::<SpriteAnimationEnabled>()
            .init_resource::<SynchronizedAnimationTimers>()
            .init_resource::<LoadedAppearances>()
            .add_event::<LoadAppearanceEvent>()
            .add_plugins(Material2dPlugin::<SpriteMaterial>::default())
            .init_resource::<RectMeshes>()
            .init_resource::<SpriteMeshes>()
            .init_resource::<TextureAtlasLayouts>()
            .add_systems(
                Update,
                initialize_sprite_material.in_set(SpriteSystems::Initialize),
            )
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

        embed_sprite_assets(app);
    }
}
