use crate::prelude::SpriteLayout;
use bevy_asset::Handle;
use bevy_ecs::prelude::Resource;
use bevy_render::mesh::Mesh;
use bevy_utils::HashMap;
use derive_more::{Deref, DerefMut};

#[derive(Resource, Debug, Clone, Default, Deref, DerefMut)]
pub struct SpriteMeshes(pub HashMap<SpriteLayout, Handle<Mesh>>);

#[derive(Resource, Debug, Clone, Default, Deref, DerefMut)]
pub struct RectMeshes(pub HashMap<SpriteLayout, Handle<Mesh>>);
