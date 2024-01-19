/*
 * Ryot - A free and open-source MMORPG server emulator
 * Copyright (©) 2023 Lucas Grossi <lucas.ggrossi@gmail.com>
 * Repository: https://github.com/lgrossi/Ryot
 * License: https://github.com/lgrossi/Ryot/blob/main/LICENSE
 * Contributors: https://github.com/lgrossi/Ryot/graphs/contributors
 * Website: https://github.com/lgrossi/Ryot
 */
use bevy::prelude::Resource;
use config::Config;
use log::error;
use ryot::cip_content::get_decompressed_file_name;
use serde::de::Visitor;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Resource)]
#[allow(unused)]
pub struct Settings {
    pub debug: bool,
    pub content: Content,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            debug: false,
            content: Content::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(unused)]
pub struct Content {
    pub path: String,
    pub catalog_name: String,
    pub decompressed_cache: DecompressedCache,
}

impl Content {
    pub fn build_content_file_path(&self, file: &String) -> String {
        format!("{}/{}", self.path, file)
    }

    pub fn build_asset_path(&self, file: &String) -> String {
        let DecompressedCache::Path(decompressed_path) = &self.decompressed_cache else {
            panic!("invalid path");
        };

        let parts: Vec<&str> = decompressed_path.split("assets/").collect();

        if parts.len() < 1 {
            panic!("decompressed path must be within assets/ folder");
        }

        format!("{}/{}", parts[1], get_decompressed_file_name(file))
    }
}

impl Default for Content {
    fn default() -> Self {
        Content {
            path: "assets/cip-catalog".to_owned(),
            catalog_name: "catalog-content.json".to_owned(),
            decompressed_cache: DecompressedCache::default(),
        }
    }
}

#[derive(Debug)]
pub enum DecompressedCache {
    Disabled,
    Path(String),
}

impl Default for DecompressedCache {
    fn default() -> Self {
        DecompressedCache::Path("assets/sprite-sheets".to_owned())
    }
}

impl serde::Serialize for DecompressedCache {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            DecompressedCache::Disabled => serializer.serialize_bool(false),
            DecompressedCache::Path(ref path) => serializer.serialize_str(path),
        }
    }
}

impl<'de> Deserialize<'de> for DecompressedCache {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DecompressedCacheVisitor;

        impl<'de> Visitor<'de> for DecompressedCacheVisitor {
            type Value = DecompressedCache;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a boolean or a string")
            }

            fn visit_bool<E>(self, value: bool) -> Result<DecompressedCache, E>
            where
                E: de::Error,
            {
                Ok(if value {
                    DecompressedCache::default()
                } else {
                    DecompressedCache::Disabled
                })
            }

            fn visit_str<E>(self, value: &str) -> Result<DecompressedCache, E>
            where
                E: de::Error,
            {
                Ok(DecompressedCache::Path(value.to_owned()))
            }
        }

        deserializer.deserialize_any(DecompressedCacheVisitor)
    }
}

pub fn build() -> Settings {
    Config::builder()
        .add_source(config::File::from_str(
            &serde_json::to_string(&Settings::default()).unwrap(),
            config::FileFormat::Json,
        ))
        .add_source(config::File::with_name("target/config/custom").required(false))
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap_or_else(|err| {
            error!("Couldn't load custom configs: {}", err.to_string());

            Settings::default()
        })
}
