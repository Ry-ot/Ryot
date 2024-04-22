use bevy::prelude::*;
use ryot_assets::prelude::*;

/// Reads the VisualElements and stores it in the VisualElements resource, removing the asset.
/// Since VisualElements is a single resource, there is no need to keep the asset handle around,
/// allowing direct access to the VisualElements resource.
pub(crate) fn prepare_visual_elements(
    mut visual_elements_res: ResMut<VisualElements>,
    mut visual_elements_assets: ResMut<Assets<VisualElements>>,
) {
    debug!("Preparing visual elements");
    let Some((key, visual_elements)) = visual_elements_assets.iter().next() else {
        panic!("No visual elements found")
    };

    *visual_elements_res = visual_elements.clone();
    visual_elements_assets.remove(key);

    debug!("Visual elements prepared");
}
