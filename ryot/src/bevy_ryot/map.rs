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
pub struct MapTiles(pub HashMap<TilePosition, HashMap<Layer, Entity>>);

#[derive(Debug, Default, Clone, Reflect)]
pub struct MapTile {
    ground: Option<Entity>,
    edge: Option<Entity>,
    bottom: HashMap<RelativeLayer, Vec<Entity>>,
    top: Option<Entity>,
}

pub struct MapTileIter<'a> {
    map_tile: &'a MapTile,
    layer: Layer,
}

impl<'a> Iterator for MapTileIter<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        let entity = self.map_tile.peek_layer(self.layer);
        if entity.is_none() {
            self.layer = self.layer.next()?;
            return self.next();
        }
        self.layer = match self.layer {
            Layer::Bottom(mut bottom_layer) => bottom_layer
                .next()
                .map(Layer::Bottom)
                .or_else(|| self.layer.next())?,
            _ => self.layer.next()?,
        };
        if entity.is_none() {
            self.next()
        } else {
            entity
        }
    }
}

impl<'a> IntoIterator for &'a MapTile {
    type Item = Entity;
    type IntoIter = MapTileIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MapTileIter {
            map_tile: self,
            layer: Layer::default(),
        }
    }
}

impl MapTile {
    pub fn peek(&self) -> Option<Entity> {
        self.into_iter().last()
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
            if let Some(entity) = self.pop_layer(layer) {
                return Some(entity);
            }
        }
        None
    }

    pub fn peek_layer(&self, layer: Layer) -> Option<Entity> {
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

    pub fn push_layer(&mut self, layer: Layer, entity: Entity) {
        match layer {
            Layer::Ground => self.ground = Some(entity),
            Layer::Edge => self.edge = Some(entity),
            Layer::Bottom(BottomLayer {
                order: Order::MAX,
                relative_layer,
            }) => self.push_bottom(relative_layer, entity),
            Layer::Bottom(BottomLayer {
                order,
                relative_layer,
            }) => self.insert_bottom(relative_layer, order, entity),
            Layer::Top => self.top = Some(entity),
            Layer::Hud(_) => (),
        }
    }

    pub fn pop_layer(&mut self, layer: Layer) -> Option<Entity> {
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

    fn push_bottom(&mut self, relative_layer: RelativeLayer, entity: Entity) {
        self.bottom.entry(relative_layer).or_default().push(entity);
    }

    fn insert_bottom(&mut self, relative_layer: RelativeLayer, order: Order, entity: Entity) {
        let v = self.bottom.entry(relative_layer).or_default();
        if order as usize >= v.len() {
            v.insert(order as usize, entity);
        } else {
            v.push(entity);
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
        map_tile.push_layer(Layer::Ground, entity);
        assert_eq!(map_tile.peek_layer(Layer::Ground), Some(entity));
        assert_eq!(map_tile.peek_layer(Layer::Edge), None);
        assert_eq!(
            map_tile.peek_layer(Layer::Bottom(BottomLayer::new(0, RelativeLayer::Creature))),
            None
        );
        assert_eq!(map_tile.peek_layer(Layer::Top), None);
        assert_eq!(map_tile.pop_layer(Layer::Ground), Some(entity));
        assert_eq!(map_tile.peek_layer(Layer::Ground), None);
        assert_eq!(map_tile.peek_layer(Layer::Edge), None);
        assert_eq!(
            map_tile.peek_layer(Layer::Bottom(BottomLayer::new(0, RelativeLayer::Creature))),
            None
        );
        assert_eq!(map_tile.peek_layer(Layer::Top), None);
    }

    #[test]
    fn test_map_tile_push_defaults() {
        let mut map_tile = MapTile::default();
        let entity = Entity::from_raw(0);
        map_tile.push_layer(
            Layer::Bottom(BottomLayer::stack(RelativeLayer::Creature)),
            entity,
        );
        assert_eq!(
            map_tile.peek_layer(Layer::Bottom(BottomLayer::stack(RelativeLayer::Creature))),
            Some(entity),
        );
    }

    #[test]
    fn test_map_tile_iterator() {
        let mut map_tile = MapTile::default();
        let entity = Entity::from_raw(0);
        map_tile.push_layer(Layer::Ground, entity);
        let mut iter = map_tile.into_iter();
        assert_eq!(iter.next(), Some(entity));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_map_tile_iterator_complex() {
        let mut map_tile = MapTile::default();
        let entity = Entity::from_raw(0);
        map_tile.push_layer(Layer::Ground, entity);
        let entity = Entity::from_raw(1);
        map_tile.push_layer(Layer::Edge, entity);
        let entity = Entity::from_raw(2);
        map_tile.push_layer(Layer::Top, entity);
        let entity = Entity::from_raw(3);
        map_tile.push_layer(
            Layer::Bottom(BottomLayer::new(0, RelativeLayer::Creature)),
            entity,
        );
        let entity = Entity::from_raw(4);
        map_tile.push_layer(
            Layer::Bottom(BottomLayer::new(1, RelativeLayer::Creature)),
            entity,
        );
        let mut iter = map_tile.into_iter();
        assert_eq!(iter.next(), Some(Entity::from_raw(0)));
        assert_eq!(iter.next(), Some(Entity::from_raw(1)));
        assert_eq!(iter.next(), Some(Entity::from_raw(3)));
        assert_eq!(iter.next(), Some(Entity::from_raw(4)));
        assert_eq!(iter.next(), Some(Entity::from_raw(2)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_map_tile_peek() {
        let mut map_tile = MapTile::default();
        let entity = Entity::from_raw(0);
        map_tile.push_layer(Layer::Ground, entity);
        assert_eq!(map_tile.peek(), Some(Entity::from_raw(0)));
        let entity = Entity::from_raw(1);
        map_tile.push_layer(Layer::Edge, entity);
        assert_eq!(map_tile.peek(), Some(Entity::from_raw(1)));
        let entity = Entity::from_raw(2);
        map_tile.push_layer(Layer::Top, entity);
        assert_eq!(map_tile.peek(), Some(Entity::from_raw(2)));
        let entity = Entity::from_raw(3);
        map_tile.push_layer(
            Layer::Bottom(BottomLayer::new(0, RelativeLayer::Creature)),
            entity,
        );
        assert_eq!(map_tile.peek(), Some(Entity::from_raw(2)));
        let entity = Entity::from_raw(4);
        map_tile.push_layer(
            Layer::Bottom(BottomLayer::new(1, RelativeLayer::Creature)),
            entity,
        );

        assert_eq!(
            map_tile.peek_layer(Layer::Ground),
            Some(Entity::from_raw(0))
        );
        assert_eq!(map_tile.peek_layer(Layer::Edge), Some(Entity::from_raw(1)));
        assert_eq!(map_tile.peek_layer(Layer::Top), Some(Entity::from_raw(2)));
        assert_eq!(
            map_tile.peek_layer(Layer::Bottom(BottomLayer::new(0, RelativeLayer::Creature))),
            Some(Entity::from_raw(3))
        );
        assert_eq!(
            map_tile.peek_layer(Layer::Bottom(BottomLayer::new(1, RelativeLayer::Creature))),
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
