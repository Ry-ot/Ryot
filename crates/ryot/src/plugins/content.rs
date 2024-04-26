use bevy_app::{App, Plugin};
use bevy_asset_loader::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_ecs::prelude::{IntoSystemConfigs, OnEnter};
use ryot_internal::prelude::*;
use std::marker::PhantomData;

#[macro_export]
macro_rules! content_plugin {
    ($plugin_name:ident, $content_assets:tt) => {
        pub struct $plugin_name<C: $content_assets>(PhantomData<C>);

        impl<C: $content_assets> Default for $plugin_name<C> {
            fn default() -> Self {
                Self(PhantomData)
            }
        }
    };
}

content_plugin!(BaseContentPlugin, VisualElementsAsset);

impl<C: VisualElementsAsset + Default> Plugin for BaseContentPlugin<C> {
    fn build(&self, app: &mut App) {
        app.init_state::<RyotContentState>()
            .init_resource::<C>()
            .init_resource::<VisualElements>()
            .register_type::<TilePosition>()
            .add_loading_state(
                LoadingState::new(RyotContentState::LoadingContent)
                    .continue_to_state(RyotContentState::PreparingContent)
                    .load_collection::<C>(),
            )
            .add_systems(
                OnEnter(RyotContentState::PreparingContent),
                prepare_visual_elements::<C>,
            );

        #[cfg(feature = "tibia")]
        app.add_plugins(tibia::TibiaAssetsPlugin);
    }
}

content_plugin!(MetaContentPlugin, VisualElementsAsset);

impl<C: VisualElementsAsset + Default> Plugin for MetaContentPlugin<C> {
    fn build(&self, app: &mut App) {
        app.add_plugins(BaseContentPlugin::<C>::default())
            .add_systems(
                OnEnter(RyotContentState::PreparingContent),
                transition_to_ready.after(prepare_visual_elements::<C>),
            );
    }
}

content_plugin!(VisualContentPlugin, VisualElementsAsset);

impl<C> Plugin for VisualContentPlugin<C>
where
    C: VisualElementsAsset + CatalogAsset + AtlasLayoutsAsset + Default,
{
    fn build(&self, app: &mut App) {
        app.add_plugins(BaseContentPlugin::<C>::default())
            .configure_loading_state(
                LoadingStateConfig::new(RyotContentState::LoadingContent)
                    .with_dynamic_assets_file::<StandardDynamicAssetArrayCollection>(
                    "dynamic.atlases.ron",
                ),
            )
            .init_resource::<SpriteSheets>()
            .add_plugins(JsonAssetPlugin::<Catalog>::new(&["json"]))
            .add_systems(
                OnEnter(RyotContentState::PreparingContent),
                (
                    prepare_sprite_layouts::<C>,
                    prepare_sprite_sheets::<C>,
                    prepare_sprite_meshes,
                    transition_to_ready,
                )
                    .chain()
                    .after(prepare_visual_elements::<C>),
            );
    }
}
