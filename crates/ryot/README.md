# Ryot

<div style="text-align: center;">
<img src="https://raw.githubusercontent.com/opentibiabr/Ryot/ae87fdf207d540c901c9c03bb6bbdd0abb8027e4/ryot_compass/assets/ryot_mascot.png" width="256" height="256"  alt="Ryot! An open tibia based MMORPG library in Rust."/>
</div>

MMORPG library based on the concepts of open tibia.

Ryot is an event-driven library that provides simple utilities for building OT based games.
It is designed to be used with the [Bevy](https://bevyengine.org/) game engine.
It is currently in early development and is not yet ready for use.

Ryot is design to integrate with OpenTibia concepts, facilitating the creation
of games that intend to use CIP-like contents/assets formats, as well as some
game mechanics.

It provides a major component:

- [ContentAssets](src/bevy_ryot/mod.rs) - A collection of content assets that
  can be loaded into the game, including appearances.dat, catalog and configs.

It also provides some utilities:

- [Appearance](src/bevy_ryot/appearances.rs) - A collection of structs and utilities used to
  manipulate protobuf based appearances, including [Prost](https://docs.rs/prost-build/latest/prost_build/) generated structs
  from the appearances.proto file.
- [Bevy Helpers](src/bevy_ryot) - A collection of helpers that can be used to send async events,
  load configurations, appearances, sprites and contents as BevyAssets.
- [Compression](src/compression.rs) - A compression utility that can be used to compress
  and decompress sprite sheets.
- [ContentBuilder](src/build/content.rs) - A builder that can be used to build
  content assets from the CIP client content folder, decompressing sprite sheets and
  copying the necessary files to the assets folder.
- [Sprite Utilities](src/sprites) - Functions that can be used to decompress, manipulate
  and load sprite sheets as game assets, taking into considerations CIP-like sprite sheets
  structures.
- [Content Utilities](src/content.rs) - A collection of structs that can be used to manipulate
  contents, including configuring and loading them.
