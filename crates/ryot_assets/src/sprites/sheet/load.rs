use crate::prelude::{Catalog, CatalogAsset, SpriteSheetDataSet};
use bevy_asset::Assets;
use bevy_ecs::change_detection::{Res, ResMut};
use bevy_utils::tracing::debug;

pub fn prepare_sprite_sheets<C: CatalogAsset>(
    content_assets: Res<C>,
    mut contents: ResMut<Assets<Catalog>>,
    mut sprite_sheets: ResMut<SpriteSheetDataSet>,
) {
    debug!("Preparing sprite sheets");

    *sprite_sheets = contents
        .get(content_assets.catalog_content())
        .expect("No catalog loaded")
        .content
        .clone()
        .into();

    contents.remove(content_assets.catalog_content());

    debug!("Finished preparing sprite sheets");
}
