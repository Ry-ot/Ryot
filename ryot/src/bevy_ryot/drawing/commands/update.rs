use crate::bevy_ryot::drawing::{
    CommandState, CommandType, Deletion, DrawingBundle, ReversibleCommand,
};
use crate::bevy_ryot::map::MapTiles;
use crate::bevy_ryot::AppearanceDescriptor;
use crate::prelude::drawing::TileComponent;
use crate::prelude::TilePosition;
use crate::Layer;
use bevy::ecs::system::Command;
use bevy::prelude::*;

#[cfg(feature = "lmdb")]
use crate::bevy_ryot::lmdb::LmdbEnv;
#[cfg(feature = "lmdb")]
use crate::lmdb::{GetKey, Item, ItemRepository, ItemsFromHeedLmdb, Tile};
#[cfg(feature = "lmdb")]
use bevy::log::{error, warn};
#[cfg(feature = "lmdb")]
use std::collections::HashMap;

pub type DrawingInfo = (
    TilePosition,
    Layer,
    Visibility,
    Option<AppearanceDescriptor>,
);

impl From<DrawingBundle> for DrawingInfo {
    fn from(bundle: DrawingBundle) -> Self {
        (
            bundle.tile_pos,
            bundle.layer,
            bundle.visibility,
            Some(bundle.appearance),
        )
    }
}

#[derive(Eq, PartialEq, Component, Default, Copy, Clone)]
pub struct UpdateComponent {
    pub new: DrawingInfo,
    pub old: DrawingInfo,
    pub state: CommandState,
}

impl UpdateComponent {
    pub fn new(new: DrawingInfo, old: DrawingInfo) -> Self {
        Self {
            new,
            old,
            state: CommandState::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UpdateTileContent(pub Vec<DrawingInfo>, Vec<DrawingInfo>);

impl UpdateTileContent {
    pub fn new(new: Vec<DrawingInfo>, old: Vec<DrawingInfo>) -> Self {
        if new.len() != old.len() {
            panic!("The new and old content must have the same length");
        }

        Self(new, old)
    }

    pub fn for_new_bundle(bundles: Vec<DrawingBundle>) -> Self {
        Self::new(
            bundles
                .iter()
                .copied()
                .map(|bundle| bundle.into())
                .collect::<Vec<DrawingInfo>>(),
            bundles
                .into_iter()
                .map(|bundle| (bundle.tile_pos, bundle.layer, bundle.visibility, None))
                .collect(),
        )
    }

    pub fn revert(&self) -> Self {
        Self::new(self.1.clone(), self.0.clone())
    }
}

impl From<UpdateTileContent> for CommandType {
    fn from(command: UpdateTileContent) -> Self {
        Box::new(command)
    }
}

impl Command for UpdateTileContent {
    fn apply(self, world: &mut World) {
        let (new, old) = (self.0, self.1);

        for (index, info) in new.iter().enumerate() {
            let id = get_or_create_entity_for_info(world, info);

            world
                .entity_mut(id)
                .insert(UpdateComponent::new(*info, old[index]));
        }
    }
}

pub fn get_or_create_entity_for_info(world: &mut World, info: &DrawingInfo) -> Entity {
    let (pos, layer, ..) = info;

    let entity = world
        .resource_mut::<MapTiles>()
        .entry(*pos)
        .or_default()
        .get(layer)
        .copied();

    match entity {
        Some(entity) => entity,
        None => world.spawn_empty().id(),
    }
}

impl ReversibleCommand for UpdateTileContent {
    fn undo(&self, commands: &mut Commands) {
        commands.add(self.revert());
    }

    fn redo(&self, commands: &mut Commands) {
        commands.add(self.clone());
    }
}

pub fn apply_update(
    mut commands: Commands,
    mut tiles: ResMut<MapTiles>,
    mut q_inserted: Query<
        (Entity, &mut UpdateComponent),
        Or<(Changed<UpdateComponent>, Added<UpdateComponent>)>,
    >,
) {
    for (entity, mut update) in q_inserted.iter_mut() {
        if update.state.applied {
            continue;
        }

        let (pos, layer, visibility, appearance) = update.new;

        // If no appearance is provided, update is ended and the deletion is triggered.
        let Some(appearance) = appearance else {
            commands
                .entity(entity)
                .insert(Deletion::default())
                .remove::<UpdateComponent>();

            continue;
        };

        commands
            .entity(entity)
            .insert((pos, layer, appearance, visibility, TileComponent))
            .remove::<Deletion>();

        tiles.entry(pos).or_default().entry(layer).or_insert(entity);

        update.state.applied = true;
    }
}

pub fn persist_update(
    #[cfg(feature = "lmdb")] lmdb_env: Res<LmdbEnv>,
    mut q_inserted: Query<
        &mut UpdateComponent,
        Or<(Changed<UpdateComponent>, Added<UpdateComponent>)>,
    >,
) {
    #[cfg(feature = "lmdb")]
    {
        let mut keys = vec![];
        let mut to_draw = vec![];

        for update in q_inserted.iter_mut() {
            let (tile_pos, layer, _, appearance) = update.new;

            if update.state.persisted {
                continue;
            }

            keys.push(tile_pos.get_binary_key());
            to_draw.push((tile_pos, layer, appearance));
        }

        let item_repository = ItemsFromHeedLmdb::new(lmdb_env.clone());
        let mut new_tiles: HashMap<TilePosition, Tile> = HashMap::new();

        let tiles = item_repository.get_for_keys(keys);

        if let Err(e) = tiles {
            error!("Failed to get tiles: {}", e);
            return;
        };

        for tile in tiles.unwrap() {
            new_tiles.insert(tile.position, tile);
        }

        for (tile_pos, layer, appearance) in &to_draw {
            let tile = new_tiles
                .entry(*tile_pos)
                .or_insert(Tile::from_pos(*tile_pos));

            let Some(appearance) = appearance else {
                warn!("Updating tile with no appearance: {:?}", tile_pos);
                continue;
            };

            tile.set_item(
                Item {
                    id: appearance.id as u16,
                    attributes: vec![],
                },
                *layer,
            );
        }

        if let Err(e) = item_repository.save_from_tiles(new_tiles.into_values().collect()) {
            error!("Failed to save tile: {}", e);
        }
    }

    for mut update in q_inserted.iter_mut() {
        if update.state.persisted {
            continue;
        }

        update.state.persisted = true;
    }
}
