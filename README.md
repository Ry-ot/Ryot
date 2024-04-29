# Ryot Framework

<div style="text-align: center;">
<img src="https://raw.githubusercontent.com/opentibiabr/Ryot/ae87fdf207d540c901c9c03bb6bbdd0abb8027e4/ryot_compass/assets/ryot_mascot.png" width="256" height="256"  alt="Ryot! An open tibia based MMORPG library in Rust."/>
</div>

Welcome to the Ryot, the Rust canarY Open Tibia Framework, a robust and versatile suite of Rust crates and applications
designed specifically for
developing tiled 2D games.
Inspired by the mechanics and perspectives of games like Open Tibia and Tibia, Ryot offers specialized tools for
creating top-down 45-degree perspective games.

## Crates

- **ryot**: Serves as the central gateway to the framework, providing essential plugins, bundles, and a unified API to
  streamline game development with Bevy.
- **ryot_assets**: Manages asset loading and processing.
- **ryot_core**: Provides foundational components and systems crucial for all other functionalities.
- **ryot_internal**: Includes utilities for internal use within the framework.
- **ryot_pathfinder**: Offers pathfinding functionalities tailored for Bevy 2D.
- **ryot_ray_casting**: Implements ray casting capabilities within Bevy 2D environments.
- **ryot_sprites**: Manages sprites and animations, enhancing the visual content of games.
- **ryot_tibia**: Specializes in handling Tibia-specific legacy assets.
- **ryot_tiled**: Supports tile-based game development with advanced map editing tools.
- **ryot_utils**: Provides general utilities and helpers used across the framework.

## Applications

- **ryot_assets_cli**: A CLI tool for asset management, supporting tasks like asset conversion and optimization.
- **ryot_compass**: A comprehensive map editor designed to streamline the creation and editing of tiled maps,
  integrating seamlessly with `ryot_tiled`.

## Getting Started

To begin using the Ryot framework and its applications, clone this repository and explore the documentation of each
component. Each module is designed for independent use yet integrates fully with others, allowing for flexible adoption
based on project needs.

## Contribution

Contributions are welcome! If you're interested in enhancing the Ryot framework or have suggestions for new features,
please refer to our contribution guidelines for more information.

Thank you for choosing the Ryot Framework for your 2D game development journey!

## License

The `Ryot` framework, is released under the GNU-APGLv3 license. For more information, the [license](LICENSE) file is
available in the root directory of this repository.