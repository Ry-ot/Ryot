use bevy_app::{App, Plugin, PostUpdate, Update};
use bevy_ecs::prelude::*;
use bevy_render::view::{check_visibility, VisibilitySystems};
use bevy_sprite::Material2dPlugin;
use ryot_content::prelude::*;
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
        app.add_plugins(Material2dPlugin::<SpriteMaterial>::default())
            .init_resource::<RectMeshes>()
            .init_resource::<SpriteMeshes>()
            .init_resource::<TextureAtlasLayouts>()
            .add_systems(
                Update,
                initialize_sprite_material.in_set(SpriteSystems::Initialize),
            );

        embed_sprite_assets(app);
    }
}
