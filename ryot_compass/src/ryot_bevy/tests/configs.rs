use super::super::*;
use bevy::prelude::*;
use rstest::{fixture, rstest};
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

#[rstest]
fn test_plugin_setup(#[from(get_test_config_app)] app: App) {
    let result = (|| -> Option<()> {
        let config_asset = get_test_config_handle(&app)?;
        assert_eq!(config_asset.source, None);

        Some(())
    })();

    assert_eq!(result, Some(()));
}

#[rstest]
fn test_load_config_dynamically(#[from(get_test_config_app)] mut app: App) {
    let result = (|| -> Option<()> {
        app.world
            .resource_mut::<Events<LoadConfigCommand<TestConfig>>>()
            .send(LoadConfigCommand::update(TestConfig {
                test: "test".to_string(),
            }));

        app.update();

        let config_asset = get_test_config_handle(&app)?;

        assert_eq!(config_asset.source, None);
        assert_eq!(get_test_config(&app)?.0.test, "test".to_string());

        Some(())
    })();

    assert_eq!(result, Some(()));
}

#[rstest]
fn test_load_config_from_file(#[from(get_test_config_app)] mut app: App) {
    let result = (|| -> Option<()> {
        app.add_plugins(TaskPoolPlugin::default());
        app.world
            .resource_mut::<Events<LoadConfigCommand<TestConfig>>>()
            .send(LoadConfigCommand::load("fixtures/.test.toml".to_string()));

        app.update();

        let config_asset = get_test_config_handle(&app)?;
        assert_eq!(config_asset.source, Some("fixtures/.test.toml".to_string()));

        Some(())
    })();

    assert_eq!(result, Some(()));
}

#[rstest]
fn test_reload_config(#[from(get_test_config_app)] mut app: App) {
    let result = (|| -> Option<()> {
        app.add_plugins(TaskPoolPlugin::default());
        app.world
            .resource_mut::<Events<LoadConfigCommand<TestConfig>>>()
            .send(LoadConfigCommand::load("fixtures/.test.toml".to_string()));

        app.update();

        let config_asset = get_test_config_handle(&app)?;
        assert_eq!(config_asset.source, Some("fixtures/.test.toml".to_string()));

        app.world
            .resource_mut::<Events<LoadConfigCommand<TestConfig>>>()
            .send(LoadConfigCommand::reload());

        app.update();

        let config_asset = get_test_config_handle(&app)?;
        assert_eq!(config_asset.source, Some("fixtures/.test.toml".to_string()));

        Some(())
    })();

    assert_eq!(result, Some(()));
}

fn get_test_config_handle(app: &App) -> Option<&ConfigHandle<TestConfig>> {
    app.world.get_resource::<ConfigHandle<TestConfig>>()
}

fn get_test_config(app: &App) -> Option<&ConfigAsset<TestConfig>> {
    app.world
        .get_resource::<Assets<ConfigAsset<TestConfig>>>()?
        .get(
            app.world
                .get_resource::<ConfigHandle<TestConfig>>()?
                .handle
                .id(),
        )
}

#[fixture]
fn get_test_config_app() -> App {
    let mut app = App::new();

    app.add_plugins(AssetPlugin::default())
        .add_plugins(ConfigPlugin::<TestConfig>::default());

    app
}
