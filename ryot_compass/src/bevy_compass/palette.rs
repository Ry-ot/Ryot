use crate::{get_egui_parameters_for_texture, DrawingAction, PaletteState, TilesetCategory};
use bevy::asset::Assets;
use bevy::log::warn;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_egui::EguiPlugin;
use leafwing_input_manager::common_conditions::action_just_pressed;
use ryot::bevy_ryot::sprites::load_sprites;
use ryot::prelude::sprites::LoadSpriteBatch;
use ryot::prelude::*;
use std::marker::PhantomData;

pub struct PalettePlugin<C: ContentAssets>(PhantomData<C>);

impl<C: ContentAssets> PalettePlugin<C> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<C: ContentAssets> Default for PalettePlugin<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: ContentAssets> Plugin for PalettePlugin<C> {
    fn build(&self, app: &mut App) {
        app.add_optional_plugin(EguiPlugin)
            .init_resource::<Palette>()
            .init_resource::<PaletteState>()
            .add_systems(OnEnter(InternalContentState::Ready), setup_categories::<C>)
            .add_systems(
                Update,
                (
                    clear_selection.run_if(action_just_pressed(DrawingAction::ClearSelection)),
                    (update_palette_category::<C>, update_palette_items::<C>).chain(),
                )
                    .run_if(in_state(InternalContentState::Ready)),
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

fn setup_categories<C: ContentAssets>(content_assets: Res<C>, mut palettes: ResMut<Palette>) {
    let Some(objects) = content_assets
        .prepared_appearances()
        .get_group(AppearanceGroup::Object)
    else {
        warn!("Appearances were not properly prepared");
        return;
    };

    objects.iter().for_each(|(asset_id, object)| {
        palettes.add_to_category(object.into(), *asset_id);
    });
}

pub fn update_palette_category<C: ContentAssets>(
    palettes: Res<Palette>,
    content_assets: Res<C>,
    mut palette_state: ResMut<PaletteState>,
) {
    if palettes.is_empty() {
        warn!("Cannot set category: palette is still empty");
        return;
    }

    if !palette_state.category_sprites.is_empty() {
        return;
    }

    let sprite_ids: Vec<(u32, u32)> = palettes
        .get_for_category(&palette_state.selected_category)
        .iter()
        .filter_map(|value| {
            let sprite_id = content_assets
                .prepared_appearances()
                .get_for_group(AppearanceGroup::Object, *value)?
                .main_sprite_id;

            Some((sprite_id, *value))
        })
        .collect();

    sprite_ids.iter().for_each(|(sprite_id, content_id)| {
        palette_state
            .category_sprites
            .insert(*sprite_id, *content_id);
    });
}

pub fn update_palette_items<C: ContentAssets>(
    palettes: Res<Palette>,
    content_assets: Res<C>,
    mut palette_state: ResMut<PaletteState>,
    texture_atlases: Res<Assets<TextureAtlasLayout>>,
    mut load_sprite_batch: EventWriter<LoadSpriteBatch>,
) {
    if palettes.is_empty() {
        return;
    }

    let len = palette_state.category_sprites.len();
    let begin = palette_state.begin().min(if len < 5 { 0 } else { len - 5 });
    let end = palette_state.end().min(len);

    let mut sprite_ids = palette_state
        .category_sprites
        .keys()
        .copied()
        .collect::<Vec<_>>();

    sprite_ids.sort();

    let sprite_ids = &sprite_ids[begin..end];

    if palette_state
        .loaded_images
        .iter()
        .map(|(sprite, ..)| sprite.sprite_id)
        .collect::<Vec<_>>()
        .eq(sprite_ids)
    {
        debug!("Palette content didn't change, no need to reload images");
        return;
    }

    palette_state.loaded_images.clear();

    for sprite in load_sprites(
        AppearanceGroup::Object,
        sprite_ids,
        &content_assets,
        &mut load_sprite_batch,
    ) {
        let Some(atlas) = texture_atlases.get(sprite.atlas_layout.clone()) else {
            continue;
        };

        let Some((rect_vec2, uv)) = get_egui_parameters_for_texture(&sprite, atlas) else {
            continue;
        };

        let texture = sprite.texture.clone_weak();

        palette_state
            .loaded_images
            .push((sprite, texture, rect_vec2, uv));
    }
}

fn clear_selection(mut palette_state: ResMut<PaletteState>) {
    palette_state.selected_tile = None;
}
