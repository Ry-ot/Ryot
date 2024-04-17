use bevy::prelude::*;
use ryot_grid::prelude::*;

pub fn update_camera_visible_sector(
    mut camera_query: Query<(&mut Sector, &Transform, &OrthographicProjection), With<Camera>>,
) {
    for (mut sector, transform, projection) in camera_query.iter_mut() {
        let new_sector = Sector::from_transform_and_projection(transform, projection);

        if new_sector == *sector {
            continue;
        }

        *sector = new_sector;
    }
}
