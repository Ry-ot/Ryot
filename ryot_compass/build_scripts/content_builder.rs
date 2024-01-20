/*
 * Ryot - A free and open-source MMORPG server emulator
 * Copyright (©) 2023 Lucas Grossi <lucas.ggrossi@gmail.com>
 * Repository: https://github.com/lgrossi/Ryot
 * License: https://github.com/lgrossi/Ryot/blob/main/LICENSE
 * Contributors: https://github.com/lgrossi/Ryot/graphs/contributors
 * Website: https://github.com/lgrossi/Ryot
 */
use ryot::{
    decompress_sprite_sheets, read_content_configs, ContentConfigs, SpriteSheetConfig,
    CONTENT_CONFIG_PATH,
};
use std::fs;
use std::path::Path;

pub fn run() -> Result<(), std::io::Error> {
    let path = Path::new("assets").join(CONTENT_CONFIG_PATH);
    let path = path.to_str().unwrap();

    // Tell Cargo to rerun this build script if the config file changes
    println!("cargo:rerun-if-changed={}", path);

    let ContentConfigs {
        directories,
        sprite_sheet,
    } = read_content_configs(path);

    // Tell Cargo to rerun this build script if our content folder changes
    println!(
        "cargo:rerun-if-changed={}",
        directories.source_path.to_str().unwrap()
    );

    // Tell Cargo to rerun this build script if our destination folder changes
    println!(
        "cargo:rerun-if-changed={}",
        directories.destination_path.to_str().unwrap()
    );

    directories.source_path.try_exists().expect(
        format!(
            "Source path {} does not exist",
            directories.source_path.to_str().unwrap()
        )
        .as_str(),
    );

    copy_catalog(&directories.source_path, &directories.destination_path)?;
    copy_appearances(&directories.source_path, &directories.destination_path)?;
    decompress_sprites(
        &directories.source_path,
        &directories.destination_path,
        sprite_sheet,
    )?;

    Ok(())
}

pub fn is_path_within_root(
    destination_path: &Path,
    root_path: &Path,
) -> Result<bool, std::io::Error> {
    Ok(fs::canonicalize(destination_path)?.starts_with(&fs::canonicalize(root_path)?))
}

fn copy_catalog(source_path: &Path, destination_path: &Path) -> Result<u64, std::io::Error> {
    let file_name = "catalog-content.json";

    fs::copy(
        &source_path.join(file_name),
        &destination_path.join(file_name),
    )
}

fn copy_appearances(source_path: &Path, destination_path: &Path) -> Result<(), std::io::Error> {
    let entries = fs::read_dir(source_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                if file_name.starts_with("appearances-")
                    && file_name.ends_with(&format!(".{}", "dat"))
                {
                    let new_path = destination_path.join("appearances.dat");

                    fs::copy(&path, &new_path)?;

                    break;
                }
            }
        }
    }

    Ok(())
}

fn decompress_sprites(
    source_path: &Path,
    destination_path: &Path,
    sheet_config: SpriteSheetConfig,
) -> Result<(), std::io::Error> {
    let sprite_sheet_path = destination_path.join(ryot::SPRITE_SHEET_FOLDER);

    fs::create_dir_all(sprite_sheet_path.clone()).expect("Failed to create sprite sheets folder");

    let files = fs::read_dir(source_path)?
        .filter_map(|e| {
            if let Ok(entry) = e {
                let path = entry.path();

                if path.is_file() {
                    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                        if file_name.starts_with("sprites-")
                            && file_name.ends_with(&format!(".{}", "bmp.lzma"))
                        {
                            return Some(file_name.to_string());
                        }
                    }
                }
            }

            None
        })
        .collect::<Vec<String>>();

    decompress_sprite_sheets(
        &source_path.to_str().unwrap(),
        &sprite_sheet_path.to_str().unwrap(),
        &files,
        sheet_config,
    );

    Ok(())
}
