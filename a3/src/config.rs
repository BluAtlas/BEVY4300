// region: includes

use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy_common_assets::ron::RonAssetPlugin;

use crate::AppState;

// endregion

// region: Config Structs

#[derive(serde::Deserialize, Asset, TypePath, Resource, Debug)]
pub struct WindowConfig {
    pub size: (f32, f32),
    pub frame_limit: f32,
    pub fullscreen: bool,
}

#[derive(Resource, Debug)]
struct WindowConfigHandle(Handle<WindowConfig>);

#[derive(serde::Deserialize, Asset, TypePath, Resource, Debug)]
pub struct FontConfig {
    pub file: String,
    pub size: f32,
    pub color: (f32, f32, f32),
}

#[derive(Resource, Debug)]
struct FontConfigHandle(Handle<FontConfig>);

#[derive(serde::Deserialize, Asset, TypePath, Resource, Debug)]
pub struct PlayerConfig {
    pub starting_position: (f32, f32),
    pub bounding_box: (f32, f32),
    pub walk_speed: f32,
    pub jump_speed: f32,
    pub max_speed: f32,
    pub gravity: f32,
    pub color: (f32, f32, f32),
}

#[derive(Resource, Debug)]
struct PlayerConfigHandle(Handle<PlayerConfig>);

#[derive(serde::Deserialize, Asset, TypePath, Resource, Debug)]
pub struct AnimationConfig {
    pub blue_slime_idle: (String, f32, f32, usize, usize, u64),
}

#[derive(Resource, Debug)]
struct AnimationConfigHandle(Handle<AnimationConfig>);

#[derive(serde::Deserialize, Asset, TypePath, Resource, Debug)]
pub struct LevelConfig {
    pub tiles: Vec<(u32, u32, f32, f32)>,
}

#[derive(Resource, Debug)]
struct LevelConfigHandle(Handle<LevelConfig>);

// endregion

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RonAssetPlugin::<WindowConfig>::new(&["window.ron"]),
            RonAssetPlugin::<FontConfig>::new(&["font.ron"]),
            RonAssetPlugin::<PlayerConfig>::new(&["player.ron"]),
            RonAssetPlugin::<AnimationConfig>::new(&["animation.ron"]),
            RonAssetPlugin::<LevelConfig>::new(&["level.ron"]),
        ))
        .add_systems(Startup, setup_config_handles)
        .add_systems(
            OnEnter(AppState::Loading),
            load_config_handles_into_resources,
        )
        .add_systems(
            Update,
            wait_for_resources
                .run_if(in_state(AppState::PostLoading))
                .run_if(resource_exists::<WindowConfig>())
                .run_if(resource_exists::<FontConfig>())
                .run_if(resource_exists::<PlayerConfig>())
                .run_if(resource_exists::<AnimationConfig>())
                .run_if(resource_exists::<LevelConfig>()),
        );
    }
}

fn setup_config_handles(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Setup plugin running");
    let window_config = WindowConfigHandle(asset_server.load("config/config.window.ron"));
    commands.insert_resource(window_config);

    let font_config_handle = FontConfigHandle(asset_server.load("config/config.font.ron"));
    commands.insert_resource(font_config_handle);

    let player_config_handle = PlayerConfigHandle(asset_server.load("config/config.player.ron"));
    commands.insert_resource(player_config_handle);

    let animation_config_handle =
        AnimationConfigHandle(asset_server.load("config/config.animation.ron"));
    commands.insert_resource(animation_config_handle);

    let level_config_handle = LevelConfigHandle(asset_server.load("config/1.level.ron"));
    commands.insert_resource(level_config_handle);
}

fn load_config_handles_into_resources(
    mut commands: Commands,
    mut state: ResMut<NextState<AppState>>,
    window_handle: Res<WindowConfigHandle>,
    mut window_configs: ResMut<Assets<WindowConfig>>,
    font_handle: Res<FontConfigHandle>,
    mut font_configs: ResMut<Assets<FontConfig>>,
    player_handle: Res<PlayerConfigHandle>,
    mut player_configs: ResMut<Assets<PlayerConfig>>,
    animation_handle: Res<AnimationConfigHandle>,
    mut animation_configs: ResMut<Assets<AnimationConfig>>,
    level_handle: Res<LevelConfigHandle>,
    mut level_configs: ResMut<Assets<LevelConfig>>,
) {
    if let Some(r) = window_configs.remove(window_handle.0.id()) {
        commands.insert_resource(r);
    } else {
        error!("Failed to insert resource: WindowConfig");
    }

    if let Some(r) = font_configs.remove(font_handle.0.id()) {
        commands.insert_resource(r);
    } else {
        error!("Failed to insert resource: FontConfig");
    }

    if let Some(r) = player_configs.remove(player_handle.0.id()) {
        commands.insert_resource(r);
    } else {
        error!("Failed to insert resource: PlayerConfig");
    }

    if let Some(r) = animation_configs.remove(animation_handle.0.id()) {
        commands.insert_resource(r);
    } else {
        error!("Failed to insert resource: AnimationConfig");
    }

    if let Some(r) = level_configs.remove(level_handle.0.id()) {
        commands.insert_resource(r);
    } else {
        error!("Failed to insert resource: LevelConfig");
    }

    state.set(AppState::PostLoading);
}

fn wait_for_resources(mut state: ResMut<NextState<AppState>>) {
    state.set(AppState::InGame);
    info!("Setup plugin finished");
}
