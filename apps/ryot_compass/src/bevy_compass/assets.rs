use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use ryot::prelude::*;

ryot_asset!(
    CompassContentAssets,
    {
        #[asset(path = "ryot_mascot.png")]
        pub mascot: Handle<Image>,
    }
);
