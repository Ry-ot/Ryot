use bevy::prelude::{Deref, DerefMut, Entity, Reflect, Resource};
use bevy::utils::HashMap;
use strum::IntoEnumIterator;

use crate::layer::{BottomLayer, Order, RelativeLayer};
use crate::{layer::Layer, position::TilePosition};

/// A resource that holds the map tiles and the entities that are drawn on them.
/// An entity location is represented by the combination of a Layer and a Position.
/// The MapTiles are represented by a HashMap of TilePosition and a HashMap of Layer and Entity.
/// The MapTiles is used to keep track of the entities that are drawn on the map and their position.
#[derive(Debug, Default, Resource, Deref, Reflect, DerefMut)]
pub struct MapTiles(pub HashMap<TilePosition, MapTile>);

#[derive(Debug, Default, Clone, Reflect)]
pub struct MapTile {
    ground: Option<Entity>,
    edge: Option<Entity>,
    bottom: HashMap<RelativeLayer, Vec<Entity>>,
    top: Option<Entity>,
}

#[derive(Debug)]
pub struct MapTileIter<'a> {
    map_tile: &'a MapTile,
    layer: Option<Layer>,
    back_layer: Option<Layer>,
}

impl<'a> Iterator for MapTileIter<'a> {
    type Item = (Layer, Entity);

    fn next(&mut self) -> Option<Self::Item> {
        let mut layer = self.layer?;
        let entity = self.map_tile.peek_for_layer(layer);

        self.layer = match layer {
            Layer::Bottom(mut bottom_layer) => bottom_layer
                .next()
                .map(Layer::Bottom)
                .or_else(|| layer.next()),
            _ => layer.next(),
        };

        if entity.is_some() {
            entity.map(|entity| (layer, entity))
        } else {
            self.next()
        }
    }
}

impl DoubleEndedIterator for MapTileIter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let mut back_layer = self.back_layer?;
        let entity = self.map_tile.peek_for_layer(back_layer);

        self.back_layer = match back_layer {
            Layer::Bottom(mut bottom_layer) => bottom_layer
                .next_back()
                .map(Layer::Bottom)
                .or_else(|| back_layer.next_back()),
            _ => back_layer.next_back(),
        };

        if entity.is_some() {
            entity.map(|entity| (back_layer, entity)).or(None)
        } else {
            self.next_back()
        }
    }
}

impl<'a> IntoIterator for &'a MapTile {
    type Item = (Layer, Entity);
    type IntoIter = MapTileIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MapTileIter {
            map_tile: self,
            layer: Some(Layer::default()),
            back_layer: Some(Layer::Hud(Order::MAX)),
        }
    }
}

impl MapTile {
    pub fn peek(&self) -> Option<Entity> {
        self.into_iter().last().map(|(_, entity)| entity)
    }

    pub fn pop(&mut self) -> Option<Entity> {
        for layer in Layer::iter().rev() {
            if matches!(layer, Layer::Bottom(_)) {
                for relative_layer in RelativeLayer::iter().rev() {
                    if let Some(entity) = self.pop_bottom(relative_layer) {
                        return Some(entity);
                    }
                }
            }
            if let Some(entity) = self.pop_from_layer(layer) {
                return Some(entity);
            }
        }
        None
    }

    pub fn peek_for_layer(&self, layer: Layer) -> Option<Entity> {
        match layer {
            Layer::Ground => self.ground,
            Layer::Edge => self.edge,
            Layer::Bottom(BottomLayer {
                order: Order::MAX,
                relative_layer,
            }) => self.peek_bottom(relative_layer),
            Layer::Bottom(BottomLayer {
                order,
                relative_layer,
            }) => self.get_bottom(relative_layer, order),
            Layer::Top => self.top,
            Layer::Hud(_) => None,
        }
    }

    pub fn push_for_layer(&mut self, layer: Layer, entity: Entity) {
        match layer {
            Layer::Ground => self.ground = Some(entity),
            Layer::Edge => self.edge = Some(entity),
            Layer::Bottom(bottom) => self.push_bottom(bottom, entity),
            Layer::Top => self.top = Some(entity),
            Layer::Hud(_) => (),
        }
    }

    pub fn pop_from_layer(&mut self, layer: Layer) -> Option<Entity> {
        match layer {
            Layer::Ground => self.ground.take(),
            Layer::Edge => self.edge.take(),
            Layer::Bottom(BottomLayer {
                order: Order::MAX,
                relative_layer,
            }) => self.pop_bottom(relative_layer),
            Layer::Bottom(BottomLayer {
                order,
                relative_layer,
            }) => self.remove_bottom(relative_layer, order),
            Layer::Top => self.top.take(),
            Layer::Hud(_) => None,
        }
    }

    fn pop_bottom(&mut self, relative_layer: RelativeLayer) -> Option<Entity> {
        self.bottom
            .get_mut(&relative_layer)
            .and_then(|entities| entities.pop())
    }

    fn remove_bottom(&mut self, relative_layer: RelativeLayer, order: Order) -> Option<Entity> {
        self.bottom.get_mut(&relative_layer).and_then(|entities| {
            if entities.len() <= order as usize {
                None
            } else {
                Some(entities.remove(order as usize))
            }
        })
    }

    fn peek_bottom(&self, relative_layer: RelativeLayer) -> Option<Entity> {
        self.bottom
            .get(&relative_layer)
            .and_then(|entities| entities.last().copied())
    }

    fn get_bottom(&self, relative_layer: RelativeLayer, order: Order) -> Option<Entity> {
        self.bottom
            .get(&relative_layer)
            .and_then(|entities| entities.get(order as usize).copied())
    }

    fn push_bottom(&mut self, bottom_layer: BottomLayer, entity: Entity) {
        let entities = self.bottom.entry(bottom_layer.relative_layer).or_default();

        match bottom_layer.order {
            Order::MAX if entities.len() >= Order::MAX as usize => {
                entities.insert(Order::MAX as usize, entity)
            }
            0..=Order::MAX => entities.push(entity),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::layer::{BottomLayer, Layer, RelativeLayer};
    use bevy::prelude::Entity;

    #[test]
    fn test_map_tile() {
        let mut map_tile = MapTile::default();
        let entity = Entity::from_raw(0);
        map_tile.push_for_layer(Layer::Ground, entity);
        assert_eq!(map_tile.peek_for_layer(Layer::Ground), Some(entity));
        assert_eq!(map_tile.peek_for_layer(Layer::Edge), None);
        assert_eq!(
            map_tile.peek_for_layer(Layer::Bottom(BottomLayer::new(0, RelativeLayer::Creature))),
            None
        );
        assert_eq!(map_tile.peek_for_layer(Layer::Top), None);
        assert_eq!(map_tile.pop_from_layer(Layer::Ground), Some(entity));
        assert_eq!(map_tile.peek_for_layer(Layer::Ground), None);
        assert_eq!(map_tile.peek_for_layer(Layer::Edge), None);
        assert_eq!(
            map_tile.peek_for_layer(Layer::Bottom(BottomLayer::new(0, RelativeLayer::Creature))),
            None
        );
        assert_eq!(map_tile.peek_for_layer(Layer::Top), None);
    }

    #[test]
    fn test_map_tile_push_defaults() {
        let mut map_tile = MapTile::default();
        let entity = Entity::from_raw(0);
        map_tile.push_for_layer(
            Layer::Bottom(BottomLayer::stack(RelativeLayer::Creature)),
            entity,
        );
        assert_eq!(
            map_tile.peek_for_layer(Layer::Bottom(BottomLayer::stack(RelativeLayer::Creature))),
            Some(entity),
        );
    }

    #[test]
    fn test_map_tile_iterator() {
        let mut map_tile = MapTile::default();
        let entity = Entity::from_raw(0);
        map_tile.push_for_layer(Layer::Ground, entity);
        let mut iter = map_tile.into_iter();

        assert_eq!(iter.next(), Some((Layer::Ground, entity)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_map_tile_iterator_complex() {
        let mut map_tile = MapTile::default();
        let entity = Entity::from_raw(0);
        map_tile.push_for_layer(Layer::Ground, entity);
        let entity = Entity::from_raw(1);
        map_tile.push_for_layer(Layer::Edge, entity);
        let entity = Entity::from_raw(2);
        map_tile.push_for_layer(Layer::Top, entity);
        let entity = Entity::from_raw(3);
        map_tile.push_for_layer(
            Layer::Bottom(BottomLayer::new(0, RelativeLayer::Creature)),
            entity,
        );
        let entity = Entity::from_raw(4);
        map_tile.push_for_layer(
            Layer::Bottom(BottomLayer::new(1, RelativeLayer::Creature)),
            entity,
        );
        let mut iter = map_tile.into_iter();

        assert_eq!(iter.next(), Some((Layer::Ground, Entity::from_raw(0))));
        assert_eq!(iter.next(), Some((Layer::Edge, Entity::from_raw(1))));
        assert_eq!(
            iter.next(),
            Some((
                Layer::Bottom(BottomLayer::new(0, RelativeLayer::Creature)),
                Entity::from_raw(3)
            ))
        );
        assert_eq!(
            iter.next(),
            Some((
                Layer::Bottom(BottomLayer::new(1, RelativeLayer::Creature)),
                Entity::from_raw(4)
            ))
        );
        assert_eq!(iter.next(), Some((Layer::Top, Entity::from_raw(2))));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_map_tile_reverse_iterator() {
        let mut map_tile = MapTile::default();
        map_tile.push_for_layer(Layer::Ground, Entity::from_raw(0));
        map_tile.push_for_layer(Layer::Edge, Entity::from_raw(0));
        map_tile.push_for_layer(
            Layer::Bottom(BottomLayer::new(0, RelativeLayer::Creature)),
            Entity::from_raw(0),
        );
        map_tile.push_for_layer(
            Layer::Bottom(BottomLayer::new(4, RelativeLayer::Creature)),
            Entity::from_raw(0),
        );
        map_tile.push_for_layer(
            Layer::Bottom(BottomLayer::new(10, RelativeLayer::Creature)),
            Entity::from_raw(0),
        );
        map_tile.push_for_layer(Layer::Top, Entity::from_raw(0));

        let mut iter = map_tile.into_iter();
        assert_eq!(iter.next_back(), Some((Layer::Top, Entity::from_raw(0))));
        assert_eq!(
            iter.next_back(),
            Some((
                Layer::Bottom(BottomLayer::new(2, RelativeLayer::Creature)),
                Entity::from_raw(0)
            ))
        );
        assert_eq!(
            iter.next_back(),
            Some((
                Layer::Bottom(BottomLayer::new(1, RelativeLayer::Creature)),
                Entity::from_raw(0)
            ))
        );
        assert_eq!(
            iter.next_back(),
            Some((
                Layer::Bottom(BottomLayer::new(0, RelativeLayer::Creature)),
                Entity::from_raw(0)
            ))
        );
        assert_eq!(iter.next_back(), Some((Layer::Edge, Entity::from_raw(0))));
        assert_eq!(iter.next_back(), Some((Layer::Ground, Entity::from_raw(0))));
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_map_tile_peek() {
        let mut map_tile = MapTile::default();
        let entity = Entity::from_raw(0);
        map_tile.push_for_layer(Layer::Ground, entity);
        assert_eq!(map_tile.peek(), Some(Entity::from_raw(0)));
        let entity = Entity::from_raw(1);
        map_tile.push_for_layer(Layer::Edge, entity);
        assert_eq!(map_tile.peek(), Some(Entity::from_raw(1)));
        let entity = Entity::from_raw(2);
        map_tile.push_for_layer(Layer::Top, entity);
        assert_eq!(map_tile.peek(), Some(Entity::from_raw(2)));
        let entity = Entity::from_raw(3);
        map_tile.push_for_layer(
            Layer::Bottom(BottomLayer::new(0, RelativeLayer::Creature)),
            entity,
        );
        assert_eq!(map_tile.peek(), Some(Entity::from_raw(2)));
        let entity = Entity::from_raw(4);
        map_tile.push_for_layer(
            Layer::Bottom(BottomLayer::new(1, RelativeLayer::Creature)),
            entity,
        );

        assert_eq!(
            map_tile.peek_for_layer(Layer::Ground),
            Some(Entity::from_raw(0))
        );
        assert_eq!(
            map_tile.peek_for_layer(Layer::Edge),
            Some(Entity::from_raw(1))
        );
        assert_eq!(
            map_tile.peek_for_layer(Layer::Top),
            Some(Entity::from_raw(2))
        );
        assert_eq!(
            map_tile.peek_for_layer(Layer::Bottom(BottomLayer::new(0, RelativeLayer::Creature))),
            Some(Entity::from_raw(3))
        );
        assert_eq!(
            map_tile.peek_for_layer(Layer::Bottom(BottomLayer::new(1, RelativeLayer::Creature))),
            Some(Entity::from_raw(4))
        );

        assert_eq!(map_tile.peek(), Some(Entity::from_raw(2)));
        assert_eq!(map_tile.pop(), Some(Entity::from_raw(2)));
        assert_eq!(map_tile.peek(), Some(Entity::from_raw(4)));
        assert_eq!(map_tile.pop(), Some(Entity::from_raw(4)));
        assert_eq!(map_tile.peek(), Some(Entity::from_raw(3)));
        assert_eq!(map_tile.pop(), Some(Entity::from_raw(3)));
        assert_eq!(map_tile.peek(), Some(Entity::from_raw(1)));
        assert_eq!(map_tile.pop(), Some(Entity::from_raw(1)));
        assert_eq!(map_tile.peek(), Some(Entity::from_raw(0)));
        assert_eq!(map_tile.pop(), Some(Entity::from_raw(0)));
    }
}
