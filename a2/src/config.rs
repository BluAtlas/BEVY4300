use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy_common_assets::ron::RonAssetPlugin;

use crate::AppState;

// region: Config Structs

#[derive(serde::Deserialize, Asset, TypePath, Resource, Debug)]
pub struct WindowConfig {
    pub size: (f32, f32),
    pub frame_limit: f32,
    pub fullscreen: bool,
}

#[derive(Resource, Debug)]
struct WindowHandle(Handle<WindowConfig>);

#[derive(serde::Deserialize, Asset, TypePath, Resource, Debug)]
pub struct FontConfig {
    pub file: String,
    pub size: f32,
    pub color: (f32, f32, f32),
}

#[derive(Resource, Debug)]
struct FontHandle(Handle<FontConfig>);

#[derive(serde::Deserialize, Asset, TypePath, Resource, Debug)]
pub struct PlayerConfig {
    pub shape_radius: f32,
    pub collision_radius: f32,
    pub speed: f32,
    pub color: (f32, f32, f32),
    pub vertices: f32,
}

#[derive(Resource, Debug)]
struct PlayerHandle(Handle<PlayerConfig>);

#[derive(serde::Deserialize, Asset, TypePath, Resource, Debug)]
pub struct EnemyConfig {
    pub shape_radius: f32,
    pub collision_radius: f32,
    pub min_speed: f32,
    pub max_speed: f32,
    pub min_vertices: f32,
    pub max_vertices: f32,
    pub small_lifespan: f32,
    pub spawn_interval: f32,
}

#[derive(Resource, Debug)]
struct EnemyHandle(Handle<EnemyConfig>);

#[derive(serde::Deserialize, Asset, TypePath, Resource, Debug)]
pub struct BulletConfig {
    pub shape_radius: f32,
    pub collision_radius: f32,
    pub speed: f32,
    pub color: (f32, f32, f32),
    pub vertices: f32,
    pub lifespan: f32,
}

#[derive(Resource, Debug)]
struct BulletHandle(Handle<BulletConfig>);

// endregion

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RonAssetPlugin::<WindowConfig>::new(&["window.ron"]),
            RonAssetPlugin::<FontConfig>::new(&["font.ron"]),
            RonAssetPlugin::<PlayerConfig>::new(&["player.ron"]),
            RonAssetPlugin::<EnemyConfig>::new(&["enemy.ron"]),
            RonAssetPlugin::<BulletConfig>::new(&["bullet.ron"]),
        ))
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::Loading), load_resources);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let window_config = WindowHandle(asset_server.load("config/config.window.ron"));
    commands.insert_resource(window_config);

    let font_config = FontHandle(asset_server.load("config/config.font.ron"));
    commands.insert_resource(font_config);

    let player_config = PlayerHandle(asset_server.load("config/config.player.ron"));
    commands.insert_resource(player_config);

    let enemy_config = EnemyHandle(asset_server.load("config/config.enemy.ron"));
    commands.insert_resource(enemy_config);

    let bullet_config = BulletHandle(asset_server.load("config/config.bullet.ron"));
    commands.insert_resource(bullet_config);
}

fn load_resources(
    mut commands: Commands,
    mut state: ResMut<NextState<AppState>>,
    window_handle: Res<WindowHandle>,
    mut window_configs: ResMut<Assets<WindowConfig>>,
    font_handle: Res<FontHandle>,
    mut font_configs: ResMut<Assets<FontConfig>>,
    player_handle: Res<PlayerHandle>,
    mut player_configs: ResMut<Assets<PlayerConfig>>,
    enemy_handle: Res<EnemyHandle>,
    mut enemy_configs: ResMut<Assets<EnemyConfig>>,
    bullet_handle: Res<BulletHandle>,
    mut bullet_configs: ResMut<Assets<BulletConfig>>,
) {
    if let Some(r) = window_configs.remove(window_handle.0.id()) {
        commands.insert_resource(r);
    }

    if let Some(r) = font_configs.remove(font_handle.0.id()) {
        commands.insert_resource(r);
    }

    if let Some(r) = player_configs.remove(player_handle.0.id()) {
        commands.insert_resource(r);
    }

    if let Some(r) = enemy_configs.remove(enemy_handle.0.id()) {
        commands.insert_resource(r);
    }

    if let Some(r) = bullet_configs.remove(bullet_handle.0.id()) {
        commands.insert_resource(r);
    }

    state.set(AppState::InGame);
}
