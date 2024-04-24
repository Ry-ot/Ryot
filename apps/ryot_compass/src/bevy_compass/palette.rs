use crate::{
    get_egui_parameters_for_texture, CompassAction, CursorCommand, PaletteState, TilesetCategory,
};
use bevy::log::warn;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_egui::EguiPlugin;
use leafwing_input_manager::common_conditions::action_just_pressed;
use ryot::bevy_ryot::sprites::{LoadAppearanceEvent, LoadedAppearances};
use ryot::prelude::*;

pub struct PalettePlugin;

impl Plugin for PalettePlugin {
    fn build(&self, app: &mut App) {
        app.add_optional_plugin(EguiPlugin)
            .init_resource::<Palette>()
            .init_resource::<PaletteState>()
            .add_systems(OnEnter(RyotContentState::Ready), setup_categories)
            .add_systems(
                Update,
                (
                    clear_selection.run_if(action_just_pressed(CompassAction::ClearSelection)),
                    (update_palette_category, update_palette_items).chain(),
                )
                    .run_if(in_state(RyotContentState::Ready)),
            );
    }
}

#[derive(Debug, Clone, Resource, Default)]
pub struct Palette {
    tile_set: HashMap<TilesetCategory, Vec<u32>>,
}

impl Palette {
    pub fn is_empty(&self) -> bool {
        self.tile_set.is_empty()
    }

    pub fn add_to_category(&mut self, category: TilesetCategory, id: u32) {
        self.tile_set.entry(category).or_default().push(id);
    }

    pub fn get_categories(&self) -> Vec<&TilesetCategory> {
        let mut categories: Vec<_> = self.tile_set.keys().collect();
        categories.push(&TilesetCategory::Raw);
        categories.sort();
        categories
    }

    pub fn get_for_category(&self, category: &TilesetCategory) -> Vec<u32> {
        match category {
            TilesetCategory::Raw => {
                // get the merge of all arrays
                let mut merged = vec![];
                for (_, v) in self.tile_set.iter() {
                    merged.extend(v);
                }
                merged
            }
            _ => self.tile_set.get(category).unwrap().to_vec(),
        }
    }
}

fn setup_categories(visual_elements: Res<VisualElements>, mut palettes: ResMut<Palette>) {
    let Some(objects) = visual_elements.get_all_for_group(EntityType::Object) else {
        warn!("Visual elements were not properly prepared");
        return;
    };

    objects.iter().for_each(|(asset_id, object)| {
        palettes.add_to_category(object.into(), *asset_id);
    });
}

pub fn update_palette_category(palettes: Res<Palette>, mut palette_state: ResMut<PaletteState>) {
    if palettes.is_empty() {
        warn!("Cannot set category: palette is still empty");
        return;
    }

    if !palette_state.category_sprites.is_empty() {
        return;
    }

    let category = palette_state.selected_category;
    palette_state.category_sprites.extend(
        palettes
            .get_for_category(&category)
            .iter()
            .map(|content_id| GameObjectId::Object(*content_id)),
    );
}

pub fn update_palette_items(
    palettes: Res<Palette>,
    texture_atlases: Res<TextureAtlasLayouts>,
    loaded_appearances: Res<LoadedAppearances>,
    mut palette_state: ResMut<PaletteState>,
    mut events: EventWriter<LoadAppearanceEvent>,
) {
    if palettes.is_empty() {
        return;
    }

    let len = palette_state.category_sprites.len();
    let begin = palette_state.begin().min(if len < 5 { 0 } else { len - 5 });
    let end = palette_state.end().min(len);

    let mut object_ids = palette_state.category_sprites.to_vec();
    object_ids.sort();
    let object_ids = &object_ids[begin..end];

    if palette_state
        .loaded_images
        .iter()
        .map(|(object_id, ..)| *object_id)
        .collect::<Vec<_>>()
        .eq(object_ids)
    {
        debug!("Palette content didn't change, no need to reload images");
        return;
    }

    palette_state.loaded_images.clear();

    let (loaded, to_load): (Vec<GameObjectId>, Vec<GameObjectId>) =
        object_ids.iter().partition(|&object_id| {
            loaded_appearances.contains_key(&(*object_id, FrameGroup::default()))
        });

    to_load.iter().for_each(|object_id| {
        events.send(LoadAppearanceEvent {
            object_id: *object_id,
            frame_group: FrameGroup::default(),
        });
    });

    loaded.into_iter().for_each(|object_id| {
        let Some(appearance) = loaded_appearances.get(&(object_id, FrameGroup::default())) else {
            return;
        };
        let Some(sprite) = appearance.sprites.first() else {
            return;
        };
        let Some((rect_vec2, uv)) = get_egui_parameters_for_texture(sprite, &texture_atlases)
        else {
            return;
        };

        let texture = sprite.texture.clone_weak();

        palette_state
            .loaded_images
            .push((object_id, texture, rect_vec2, uv));
    });
}

fn clear_selection(
    mut palette_state: ResMut<PaletteState>,
    mut cursor_events_writer: EventWriter<CursorCommand>,
) {
    palette_state.selected_tile = None;
    cursor_events_writer.send(CursorCommand::ChangeToolMode(None));
}
