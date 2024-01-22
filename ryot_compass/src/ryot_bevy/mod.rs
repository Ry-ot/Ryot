use bevy::prelude::*;
use ryot::appearances::ContentType;
use ryot::*;

use crate::Settings;

mod appearances;
pub use appearances::*;

mod async_events;
pub use async_events::*;

mod configs;
pub use configs::*;

mod palette;
pub use palette::*;
use ryot::tile_grid::TileGrid;

#[derive(Resource, Debug, Default)]
pub struct CipContent {
    pub raw_content: Vec<ContentType>,
}

#[derive(Resource, Default)]
pub struct TextureAtlasHandlers(pub bevy::utils::HashMap<String, Handle<TextureAtlas>>);

#[derive(Debug)]
pub struct LoadedSprite {
    pub sprite_id: u32,
    pub sprite_index: usize,
    pub atlas_grid: SheetGrid,
    pub atlas_texture_handle: Handle<TextureAtlas>,
}

pub fn load_sprites(
    sprite_ids: &[u32],
    content: &[ContentType],
    settings: &Res<Settings>,
    asset_server: &Res<AssetServer>,
    atlas_handlers: &mut ResMut<TextureAtlasHandlers>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Vec<LoadedSprite> {
    let unsaved_sprites: Vec<(SheetGrid, TextureAtlas)> = sprite_ids
        .iter()
        .filter_map(|sprite_id| {
            let grid = get_sprite_grid_by_id(content, *sprite_id).ok()?;
            Some((
                grid.clone(),
                build_texture_atlas_from_sheet(&grid, settings, asset_server).unwrap(),
            ))
        })
        .collect();

    unsaved_sprites.iter().for_each(|(grid, atlas)| {
        build_atlas_handler(grid, atlas.clone(), atlas_handlers, texture_atlases);
    });

    sprite_ids
        .iter()
        .filter_map(|sprite_id| {
            let grid = get_sprite_grid_by_id(content, *sprite_id).ok()?;
            Some(LoadedSprite {
                sprite_id: *sprite_id,
                sprite_index: get_sprite_index_by_id(content, *sprite_id).ok()?,
                atlas_grid: grid.clone(),
                atlas_texture_handle: atlas_handlers.0.get(&grid.file)?.clone(),
            })
        })
        .collect::<Vec<_>>()
}

pub fn load_sprite(
    sprite_id: u32,
    content: &[ContentType],
    settings: &Res<Settings>,
    asset_server: &Res<AssetServer>,
    atlas_handlers: &mut ResMut<TextureAtlasHandlers>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    if let Ok(grid) = get_sprite_grid_by_id(content, sprite_id) {
        let atlas = build_texture_atlas_from_sheet(&grid, settings, asset_server)
            .expect("Failed to build texture atlas");
        build_atlas_handler(&grid, atlas, atlas_handlers, texture_atlases);
    }
}

pub fn get_sprite_by_id(
    content: &[ContentType],
    sprite_id: u32,
    atlas_handlers: &ResMut<TextureAtlasHandlers>,
) -> Option<LoadedSprite> {
    let grid = get_sprite_grid_by_id(content, sprite_id).ok()?;
    Some(LoadedSprite {
        sprite_id,
        sprite_index: get_sprite_index_by_id(content, sprite_id).ok()?,
        atlas_grid: grid.clone(),
        atlas_texture_handle: atlas_handlers.0.get(&grid.file)?.clone(),
    })
}

pub fn build_atlas_handler(
    grid: &SheetGrid,
    texture_atlas: TextureAtlas,
    atlas_handlers: &mut ResMut<TextureAtlasHandlers>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Handle<TextureAtlas> {
    if !atlas_handlers.0.contains_key(&grid.file) {
        let atlas_handle = texture_atlases.add(texture_atlas);
        atlas_handlers.0.insert(grid.file.clone(), atlas_handle);
    }

    atlas_handlers
        .0
        .get(&grid.file)
        .expect(
            "Failed to get atlas handler, this should never happen, please report this as a bug",
        )
        .clone()
}

pub fn build_texture_atlas_from_sheet(
    grid: &SheetGrid,
    settings: &Res<Settings>,
    asset_server: &Res<AssetServer>,
) -> Result<TextureAtlas, std::io::Error> {
    // let DecompressedCache::Path(decompressed_path) = &settings.content.decompressed_cache else {
    //     return Err(std::io::Error::new(
    //         std::io::ErrorKind::Other,
    //         "invalid path",
    //     ));
    // };
    //
    // #[cfg(not(target_arch = "wasm32"))]
    // {
    //     std::fs::create_dir_all(decompressed_path)?;
    //
    //     let path = decompressed_path.join(PathBuf::from(&grid.file));
    //
    //     if !path.exists() {
    //         decompress_sprite_sheet(
    //             &grid.file,
    //             &settings.content.path,
    //             decompressed_path,
    //             cip_sheet(),
    //         );
    //     }
    // }

    let image_handle: Handle<Image> =
        asset_server.load(settings.content.build_asset_path(&grid.file));

    Ok(TextureAtlas::from_grid(
        image_handle,
        Vec2::new(grid.tile_size.x as f32, grid.tile_size.y as f32),
        grid.columns,
        grid.rows,
        None,
        None,
    ))
}

pub fn draw_sprite(pos: Vec3, sprite: &LoadedSprite, commands: &mut Commands) {
    let Some(tile_pos) = TileGrid::default().get_display_position_from_tile_pos_vec3(pos) else {
        return;
    };

    commands.spawn(build_sprite_bundle(
        sprite.atlas_texture_handle.clone(),
        tile_pos,
        sprite.sprite_index,
    ));
}

pub fn build_sprite_bundle(
    handle: Handle<TextureAtlas>,
    translation: Vec3,
    index: usize,
) -> SpriteSheetBundle {
    SpriteSheetBundle {
        transform: Transform {
            translation,
            ..default()
        },
        sprite: TextureAtlasSprite::new(index),
        texture_atlas: handle,
        ..default()
    }
}
