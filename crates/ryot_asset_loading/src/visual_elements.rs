use bevy_asset::Assets;
use bevy_asset_loader::prelude::AssetCollection;
use bevy_ecs::change_detection::{Res, ResMut};
use bevy_ecs::prelude::Resource;
use bevy_utils::tracing::debug;
use ryot_content::prelude::VisualElements;

pub trait VisualElementsAsset: Resource + AssetCollection + Send + Sync + 'static {
    fn visual_elements(&self) -> &bevy_asset::Handle<VisualElements>;
}

/// Reads the VisualElements and stores it in the VisualElements resource, removing the asset.
/// Since VisualElements is a single resource, there is no need to keep the asset handle around,
/// allowing direct access to the VisualElements resource.
pub fn prepare_visual_elements<C: VisualElementsAsset>(
    content_assets: Res<C>,
    mut visual_elements_res: ResMut<VisualElements>,
    mut visual_elements_assets: ResMut<Assets<VisualElements>>,
) {
    debug!("Preparing visual elements");

    let key = content_assets.visual_elements();

    let Some(visual_elements) = visual_elements_assets.get(key) else {
        panic!("No visual elements found")
    };

    *visual_elements_res = visual_elements.clone();
    visual_elements_assets.remove(key);

    debug!("Visual elements prepared");
}
