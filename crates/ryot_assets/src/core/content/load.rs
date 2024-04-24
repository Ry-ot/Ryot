use bevy_asset::{Asset, Handle};
use bevy_ecs::change_detection::ResMut;
use bevy_ecs::prelude::{NextState, States};
use bevy_reflect::TypePath;

/// The states that the content loading process can be in.
/// This is used to track the progress of the content loading process.
/// It's also used to determine if the content is ready to be used.
/// It's internally used by the `ContentPlugin` and should not be manipulated directly.
/// Can be checked by applications to perform actions that depend on the state of the content.
#[derive(Default, Clone, Eq, PartialEq, Debug, Hash, States)]
pub enum InternalContentState {
    #[default]
    LoadingContent,
    PreparingContent,
    Ready,
}

pub fn transition_to_ready(mut state: ResMut<NextState<InternalContentState>>) {
    state.set(InternalContentState::Ready);
}

pub trait CatalogAsset: crate::core::AssetResource {
    fn catalog_content(&self) -> &Handle<Catalog>;
}

/// An asset that holds a collection of raw content configs.
#[derive(serde::Deserialize, TypePath, Asset)]
#[serde(transparent)]
pub struct Catalog {
    pub content: Vec<crate::prelude::ContentType>,
}
