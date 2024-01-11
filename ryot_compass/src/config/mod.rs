/*
 * Ryot - A free and open-source MMORPG server emulator
 * Copyright (©) 2023 Lucas Grossi <lucas.ggrossi@gmail.com>
 * Repository: https://github.com/lgrossi/Ryot
 * License: https://github.com/lgrossi/Ryot/blob/main/LICENSE
 * Contributors: https://github.com/lgrossi/Ryot/graphs/contributors
 * Website: https://github.com/lgrossi/Ryot
 */
use config::{Config, ConfigError};
use serde::de::Visitor;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

mod bevy;
pub use bevy::*;

#[derive(Debug, Serialize, Deserialize)]
#[allow(unused)]
pub struct Settings {
    debug: bool,
    content: Content,
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
    path: String,
    decompressed_cache: DecompressedCache,
}

impl Default for Content {
    fn default() -> Self {
        Content {
            path: "assets/cip_catalog".to_owned(),
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

pub fn config() -> Result<Settings, ConfigError> {
    let default_json = serde_json::to_string(&Settings::default()).unwrap();

    Config::builder()
        .add_source(config::File::from_str(
            &default_json,
            config::FileFormat::Json,
        ))
        .add_source(config::File::with_name("target/config/custom").required(false))
        .build()?
        .try_deserialize()
}
