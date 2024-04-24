use bevy::prelude::*;

#[cfg(all(feature = "bevy", feature = "debug"))]
use bevy_stroked_text::StrokedText;
use ryot_content::prelude::Elevation;

#[cfg(feature = "bevy")]
use ryot_content::prelude::SpriteLayout;
use ryot_tiled::prelude::*;

#[cfg(feature = "debug")]
#[derive(Component)]
pub struct PositionDebugText;

#[cfg(feature = "debug")]
pub fn debug_y_offset(layer: &Layer) -> f32 {
    (tile_size().as_vec2().y / 24.)
        * match layer {
            Layer::Ground => 0.,
            Layer::Edge => 1.,
            Layer::Bottom(layer) => match layer.relative_layer {
                RelativeLayer::Object => 2.,
                RelativeLayer::Creature => 3.,
                RelativeLayer::Effect => 4.,
                RelativeLayer::Missile => 5.,
            },
            Layer::Top => 6.,
            Layer::Hud(_) => 7.,
        }
        - tile_size().as_vec2().y / 2.
}

#[cfg(feature = "bevy")]
type PositionChangedFilter = (
    With<Transform>,
    Or<(Added<Transform>, Changed<TilePosition>, Changed<Elevation>)>,
);

/// This system syncs the sprite position with the TilePosition.
/// Every spawned sprite has a Transform component, which is used to position the sprite on
/// the screen. However, in this library our world components are treated in terms of TilePosition.
/// So, we need to sync the sprite position with the TilePosition.
///
/// This system listen to all new and changed TilePosition components and update the Transform
/// component of the sprite accordingly, if it exist. Ideally this should run in the end of
/// the Update stage, so it can be sure that all TilePosition components have been updated.
#[cfg(feature = "bevy")]
pub fn update_sprite_position(
    mut query: Query<
        (
            &SpriteLayout,
            &TilePosition,
            &Elevation,
            &Layer,
            &mut Transform,
        ),
        (PositionChangedFilter, Without<SpriteMovement>),
    >,
) {
    query
        .par_iter_mut()
        .for_each(|(layout, tile_pos, elevation, layer, mut transform)| {
            transform.translation = elevate_position(tile_pos, *layout, *layer, *elevation);
        });
}

#[cfg(all(feature = "bevy", feature = "debug"))]
pub fn debug_sprite_position(
    mut query: Query<
        (&Elevation, &Transform, Option<&Children>),
        Or<(Changed<Transform>, Changed<Elevation>)>,
    >,
    mut children_query: Query<&mut StrokedText, With<PositionDebugText>>,
) {
    query
        .iter_mut()
        .for_each(|(elevation, transform, children)| {
            let Some(children) = children else {
                return;
            };
            children.iter().for_each(|child| {
                if let Ok(mut text) = children_query.get_mut(*child) {
                    text.text = format!("{:.02} [{}]", 1000. * transform.translation.z, elevation);
                }
            });
        });
}

#[cfg(feature = "bevy")]
pub fn move_sprites_with_animation(
    mut query: Query<(&mut Transform, &mut SpriteMovement)>,
    time: Res<Time>,
) {
    query
        .par_iter_mut()
        .for_each(|(mut transform, mut movement)| {
            movement.timer.tick(time.delta());
            // We need the moving entity to be on top of other entities
            // This is to ensure that the layering logic is consistent no matter what direction
            // the entity is moving in
            let z = movement.origin.z.max(movement.destination.z);
            transform.translation = movement
                .origin
                .lerp(movement.destination, movement.timer.fraction())
                .truncate()
                .extend(z);
        });
}

#[cfg(feature = "bevy")]
pub fn finish_position_animation(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &SpriteMovement)>,
) {
    query
        .iter_mut()
        .filter(|(_, _, movement)| movement.timer.just_finished())
        .for_each(|(entity, mut transform, movement)| {
            if movement.despawn_on_end {
                commands.entity(entity).despawn_recursive();
            } else {
                transform.translation.z = movement.destination.z;
                commands.entity(entity).remove::<SpriteMovement>();
            }
        });
}
