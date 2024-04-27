use crate::prelude::*;
use derive_more::{Deref, DerefMut};

#[cfg(feature = "bevy")]
use bevy_utils::HashMap;
#[cfg(not(feature = "bevy"))]
use std::collections::HashMap;

#[derive(Clone, PartialEq, Default, Debug, Deref, DerefMut)]
#[cfg_attr(
    feature = "bevy",
    derive(bevy_ecs::prelude::Resource, bevy_reflect::TypePath, bevy_asset::Asset)
)]
pub struct VisualElements(HashMap<ContentType, HashMap<u32, VisualElement>>);

impl VisualElements {
    pub fn get_all_for_group(&self, group: ContentType) -> Option<&HashMap<u32, VisualElement>> {
        self.get(&group)
    }

    pub fn get_for_group_and_id(&self, group: ContentType, id: u32) -> Option<&VisualElement> {
        self.get(&group)?.get(&id)
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
pub struct VisualElement {
    pub id: u32,
    pub name: String,
    pub main_sprite_id: Option<u32>,
    pub sprites_info: Vec<SpriteInfo>,
    pub flags: Flags,
    pub category: Category,
    pub properties: Properties,
}
