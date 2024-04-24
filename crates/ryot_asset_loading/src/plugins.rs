use crate::prelude::*;
use bevy_app::{App, Plugin};
use bevy_asset_loader::loading_state::LoadingState;
use bevy_asset_loader::prelude::{ConfigureLoadingState, LoadingStateAppExt};
use bevy_ecs::prelude::{IntoSystemConfigs, OnEnter};
use ryot_content::prelude::{transition_to_ready, RyotContentState, VisualElements};
use ryot_tiled::prelude::TilePosition;
use std::marker::PhantomData;

/// A plugin that registers implementations of ContentAssets and loads them.
/// It inits the necessary resources and adds the necessary systems and plugins to load
/// the content assets.
///
/// It also manages the loading state of the content assets, the lifecycle of the content
/// and the events that allow lazy loading of sprites.
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

        #[cfg(feature = "ryot_tibia_content")]
        app.add_plugins(ryot_tibia_content::prelude::TibiaAssetsPlugin);
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
