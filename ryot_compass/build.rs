extern crate embed_resource;

mod build_scripts;
pub use build_scripts::*;

fn main() {
    build_target::run();
    assets_builder::run().expect("Failed to build assets");
}
