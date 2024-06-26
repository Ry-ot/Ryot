use crate::prelude::Sector;
use bevy_ecs::prelude::*;
use bevy_render::prelude::*;
use bevy_transform::prelude::Transform;

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
