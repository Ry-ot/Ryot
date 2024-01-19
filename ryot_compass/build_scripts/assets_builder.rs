/*
 * Ryot - A free and open-source MMORPG server emulator
 * Copyright (©) 2023 Lucas Grossi <lucas.ggrossi@gmail.com>
 * Repository: https://github.com/lgrossi/Ryot
 * License: https://github.com/lgrossi/Ryot/blob/main/LICENSE
 * Contributors: https://github.com/lgrossi/Ryot/graphs/contributors
 * Website: https://github.com/lgrossi/Ryot
 */
use config::Config;
use ryot::cip_content::decompress_sprite_sheets;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

static CONFIG_PATH: &str = "config/build/Assets.toml";
static SPRITE_SHEET_FOLDER: &str = "sprite-sheets";

#[derive(Debug, Deserialize)]
pub struct AssetsConfig {
    source_path: PathBuf,
    #[serde(default = "default_destination_path")]
    destination_path: PathBuf,
}

pub fn run() -> Result<(), std::io::Error> {
    eprintln!("fuuuck");
    // Tell Cargo to rerun this build script if the config file changes
    println!("cargo:rerun-if-changed={}", CONFIG_PATH);

    let settings = read_assets_configs(CONFIG_PATH);

    // Tell Cargo to rerun this build script if our content file changes
    println!(
        "cargo:rerun-if-changed={}",
        settings.source_path.to_str().unwrap()
    );

    settings.source_path.try_exists().expect(
        format!(
            "Source path {} does not exist",
            settings.source_path.to_str().unwrap()
        )
        .as_str(),
    );

    copy_catalog(&settings.source_path, &settings.destination_path)?;
    copy_appearances(&settings.source_path, &settings.destination_path)?;
    decompress_sprites(&settings.source_path, &settings.destination_path)?;

    Ok(())
}

pub fn read_assets_configs(config_path: &str) -> AssetsConfig {
    let settings = Config::builder()
        .add_source(config::File::with_name(config_path))
        .build()
        .expect("Failed to build config")
        .try_deserialize::<AssetsConfig>()
        .expect("Failed to deserialize config");

    match is_path_within_root(&settings.destination_path, Path::new("assets")) {
        Ok(true) => settings,
        Ok(false) | Err(_) => panic!(
            "Target path {} is not within assets folder",
            settings
                .destination_path
                .to_str()
                .expect("Failed to convert target path to str")
        ),
    }
}

pub fn is_path_within_root(
    destination_path: &Path,
    root_path: &Path,
) -> Result<bool, std::io::Error> {
    Ok(fs::canonicalize(destination_path)?.starts_with(&fs::canonicalize(root_path)?))
}

fn default_destination_path() -> PathBuf {
    PathBuf::from("assets")
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

fn decompress_sprites(source_path: &Path, destination_path: &Path) -> Result<(), std::io::Error> {
    let sprite_sheet_path = destination_path.join(SPRITE_SHEET_FOLDER);

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
        &files,
        &source_path.to_str().unwrap(),
        &sprite_sheet_path.to_str().unwrap(),
    );

    Ok(())
}
