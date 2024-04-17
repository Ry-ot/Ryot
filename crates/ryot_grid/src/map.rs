use crate::layer::*;
use crate::position::TilePosition;

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "bevy")]
use bevy_reflect::Reflect;
#[cfg(feature = "bevy")]
use bevy_utils::HashMap;

#[cfg(not(feature = "bevy"))]
use std::collections::HashMap;

use derive_more::*;

/// A resource that holds the map tiles and the entities that are drawn on them.
/// An entity location is represented by the combination of a Layer and a Position.
/// The MapTiles are represented by a HashMap of TilePosition and a HashMap of Layer and Entity.
/// The MapTiles is used to keep track of the entities that are drawn on the map and their position.
#[derive(Debug, Deref, DerefMut)]
#[cfg_attr(feature = "bevy", derive(Resource))]
pub struct MapTiles<T: Copy>(pub HashMap<TilePosition, MapTile<T>>);

impl<T: Copy> Default for MapTiles<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub struct MapTile<T: Copy> {
    ground: Option<T>,
    edge: Option<T>,
    bottom: HashMap<RelativeLayer, HashMap<Order, T>>,
    top: Option<T>,
}

impl<T: Copy> Default for MapTile<T> {
    fn default() -> Self {
        Self {
            ground: None,
            edge: None,
            bottom: Default::default(),
            top: None,
        }
    }
}

#[derive(Debug)]
pub struct MapTileIter<'a, T: Copy> {
    map_tile: &'a MapTile<T>,
    layer: Option<Layer>,
    back_layer: Option<Layer>,
}

impl<'a, T: Copy> Iterator for MapTileIter<'a, T> {
    type Item = (Layer, T);

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

impl<T: Copy> DoubleEndedIterator for MapTileIter<'_, T> {
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

impl<'a, T: Copy> IntoIterator for &'a MapTile<T> {
    type Item = (Layer, T);
    type IntoIter = MapTileIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        MapTileIter {
            map_tile: self,
            layer: Some(Layer::default()),
            back_layer: Some(Layer::Hud(Order::MAX)),
        }
    }
}

impl<T: Copy> MapTile<T> {
    pub fn peek(&self) -> Option<(Layer, T)> {
        self.into_iter().last()
    }

    pub fn pop(&mut self) -> Option<T> {
        let last = self.peek()?;
        let (layer, _) = last;

        self.pop_from_layer(layer)
    }

    pub fn peek_for_layer(&self, layer: Layer) -> Option<T> {
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
            }) => self.get_bottom(relative_layer, &order),
            Layer::Top => self.top,
            Layer::Hud(_) => None,
        }
    }

    pub fn push_for_layer(&mut self, layer: Layer, entity: T) {
        match layer {
            Layer::Ground => self.ground = Some(entity),
            Layer::Edge => self.edge = Some(entity),
            Layer::Bottom(bottom) => {
                self.push_bottom(bottom, entity);
            }
            Layer::Top => self.top = Some(entity),
            Layer::Hud(_) => (),
        }
    }

    pub fn pop_from_layer(&mut self, layer: Layer) -> Option<T> {
        match layer {
            Layer::Ground => self.ground.take(),
            Layer::Edge => self.edge.take(),
            Layer::Bottom(BottomLayer {
                order,
                relative_layer,
            }) => self.remove_bottom(relative_layer, &order),
            Layer::Top => self.top.take(),
            Layer::Hud(_) => None,
        }
    }

    pub fn remove_bottom(&mut self, relative_layer: RelativeLayer, order: &Order) -> Option<T> {
        self.bottom
            .get_mut(&relative_layer)
            .and_then(|entities| entities.remove(order))
    }

    pub fn peek_bottom(&self, relative_layer: RelativeLayer) -> Option<T> {
        self.bottom
            .get(&relative_layer)
            .and_then(|entities| entities.iter().last())
            .map(|(_, entity)| *entity)
    }

    pub fn get_bottom(&self, relative_layer: RelativeLayer, order: &Order) -> Option<T> {
        self.bottom
            .get(&relative_layer)
            .and_then(|entities| entities.get(order))
            .copied()
    }

    pub fn push_bottom(&mut self, bottom_layer: BottomLayer, entity: T) -> Layer {
        let bottom = self.bottom.entry(bottom_layer.relative_layer).or_default();
        let order = if bottom_layer.order == Order::MAX {
            bottom
                .iter()
                .map(|(order, _)| order)
                .max()
                .cloned()
                .map_or_else(Order::default, |order| order + 1)
        } else {
            bottom_layer.order
        };
        bottom.insert(order, entity);
        Layer::Bottom(BottomLayer {
            order,
            relative_layer: bottom_layer.relative_layer,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bevy_ecs::prelude::Entity;

    #[test]
    fn test_map_tile() {
        let mut map_tile = MapTile::<Entity>::default();
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
        let mut map_tile = MapTile::<Entity>::default();
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
        let mut map_tile = MapTile::<Entity>::default();
        let entity = Entity::from_raw(0);
        map_tile.push_for_layer(Layer::Ground, entity);
        let mut iter = map_tile.into_iter();

        assert_eq!(iter.next(), Some((Layer::Ground, entity)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_map_tile_iterator_complex() {
        let mut map_tile = MapTile::<Entity>::default();
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
    fn test_map_tile_iterator_with_objects() {
        let mut map_tile = MapTile::<Entity>::default();
        let entity = Entity::from_raw(0);
        map_tile.push_for_layer(Layer::Ground, entity);
        let entity = Entity::from_raw(1);
        map_tile.push_for_layer(
            Layer::Bottom(BottomLayer::stack(RelativeLayer::Object)),
            entity,
        );
        let entity = Entity::from_raw(2);
        map_tile.push_for_layer(
            Layer::Bottom(BottomLayer::stack(RelativeLayer::Object)),
            entity,
        );

        let mut iter = map_tile.into_iter();
        assert_eq!(iter.next(), Some((Layer::Ground, Entity::from_raw(0))));
        assert_eq!(
            iter.next(),
            Some((
                Layer::Bottom(BottomLayer::new(0, RelativeLayer::Object)),
                Entity::from_raw(1)
            ))
        );
        assert_eq!(
            iter.next(),
            Some((
                Layer::Bottom(BottomLayer::new(1, RelativeLayer::Object)),
                Entity::from_raw(2)
            ))
        );

        assert_eq!(
            map_tile.into_iter().collect::<Vec<_>>(),
            vec![
                (Layer::Ground, Entity::from_raw(0)),
                (
                    Layer::Bottom(BottomLayer::new(0, RelativeLayer::Object)),
                    Entity::from_raw(1)
                ),
                (
                    Layer::Bottom(BottomLayer::new(1, RelativeLayer::Object)),
                    Entity::from_raw(2)
                ),
            ]
        )
    }

    #[test]
    fn test_map_tile_reverse_iterator() {
        let mut map_tile = MapTile::<Entity>::default();
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
                Layer::Bottom(BottomLayer::new(10, RelativeLayer::Creature)),
                Entity::from_raw(0)
            ))
        );
        assert_eq!(
            iter.next_back(),
            Some((
                Layer::Bottom(BottomLayer::new(4, RelativeLayer::Creature)),
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
        let mut map_tile = MapTile::<Entity>::default();
        let entity = Entity::from_raw(0);
        map_tile.push_for_layer(Layer::Ground, entity);
        assert_eq!(map_tile.peek(), Some((Layer::Ground, Entity::from_raw(0))));
        let entity = Entity::from_raw(1);
        map_tile.push_for_layer(Layer::Edge, entity);
        assert_eq!(map_tile.peek(), Some((Layer::Edge, Entity::from_raw(1))));
        let entity = Entity::from_raw(2);
        map_tile.push_for_layer(Layer::Top, entity);
        assert_eq!(map_tile.peek(), Some((Layer::Top, Entity::from_raw(2))));
        let entity = Entity::from_raw(3);
        map_tile.push_for_layer(
            Layer::Bottom(BottomLayer::new(0, RelativeLayer::Creature)),
            entity,
        );
        assert_eq!(map_tile.peek(), Some((Layer::Top, Entity::from_raw(2))));
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

        assert_eq!(map_tile.peek(), Some((Layer::Top, Entity::from_raw(2))));
        assert_eq!(map_tile.pop(), Some(Entity::from_raw(2)));
        assert_eq!(
            map_tile.peek(),
            Some((
                Layer::Bottom(BottomLayer::new(1, RelativeLayer::Creature)),
                Entity::from_raw(4)
            ))
        );
        assert_eq!(map_tile.pop(), Some(Entity::from_raw(4)));
        assert_eq!(
            map_tile.peek(),
            Some((
                Layer::Bottom(BottomLayer::new(0, RelativeLayer::Creature)),
                Entity::from_raw(3)
            ))
        );
        assert_eq!(map_tile.pop(), Some(Entity::from_raw(3)));
        assert_eq!(map_tile.peek(), Some((Layer::Edge, Entity::from_raw(1))));
        assert_eq!(map_tile.pop(), Some(Entity::from_raw(1)));
        assert_eq!(map_tile.peek(), Some((Layer::Ground, Entity::from_raw(0))));
        assert_eq!(map_tile.pop(), Some(Entity::from_raw(0)));
    }
}
