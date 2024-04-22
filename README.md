# Ryot

<div style="text-align: center;">
<img src="https://raw.githubusercontent.com/opentibiabr/Ryot/ae87fdf207d540c901c9c03bb6bbdd0abb8027e4/ryot_compass/assets/ryot_mascot.png" width="256" height="256"  alt="Ryot! An open tibia based MMORPG library in Rust."/>
</div>

MMORPG library based on the concepts of open tibia.

Ryot is an event-driven library that provides simple utilities for building OT based games.
It is designed to be used with the [Bevy](https://bevyengine.org/) game engine.
It is currently in early development and is not yet ready for use.

Ryot is design to integrate with OpenTibia concepts, facilitating the creation
of games that intend to use Tibia-like contents/assets formats, as well as some
game mechanics.

It also provides some utilities:

- [Bevy Helpers](https://github.com/opentibiabr/Ryot/tree/main/ryot/src/bevy_ryot) - A collection of helpers that can be
  used to send async events,
  load configurations, sprites and contents as BevyAssets.
- [Compression](https://github.com/opentibiabr/Ryot/blob/main/ryot/src/compression.rs) - A compression utility that can
  be used to compress
  and decompress sprite sheets.
- [ContentBuilder](https://github.com/opentibiabr/Ryot/blob/main/ryot/src/build/content.rs) - A builder that can be used
  to build
  content assets from the Tibia client content folder, decompressing sprite sheets and
  copying the necessary files to the assets folder.
- [Sprite Utilities](https://github.com/opentibiabr/Ryot/tree/main/ryot/src/sprites) - Functions that can be used to
  decompress, manipulate
  and load sprite sheets as game assets, taking into considerations Tibia-like sprite sheets
  structures.
- [Content Utilities](https://github.com/opentibiabr/Ryot/blob/main/ryot/src/content.rs) - A collection of structs that
  can be used to manipulate
  contents, including configuring and loading them.

# Compass

Map editor built on top of Ryot aiming develop tile based map for open tibia based MMORPGs.
