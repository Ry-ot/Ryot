use super::super::*;
use bevy::prelude::*;
use rstest::fixture;
use serde::Deserialize;

#[derive(Default, Deserialize, Clone, Debug)]
pub struct TestConfig {
    pub test: String,
}

impl Configurable for TestConfig {
    fn extensions() -> Vec<&'static str> {
        vec!["test.toml"]
    }
}

#[fixture]
fn get_test_config_app() -> App {
    let mut app = App::new();

    app.add_plugins(AssetPlugin::default())
        .add_plugins(ConfigPlugin::<TestConfig>::default());

    app
}
