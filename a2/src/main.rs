// region: includes

use std::{f32::consts::PI, thread::spawn};

use bevy::{prelude::*, window::*};
use bevy_rand::{plugin::EntropyPlugin, prelude::WyRand, resource::GlobalEntropy};
use config::*;
use rand_core::RngCore;

mod config;

// endregion

// region: AppState

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Loading,
    InGame,
}

// endregion

// region: Components

#[derive(Component)]
struct CTransform {
    pos: Vec2,
    vel: Vec2,
    angle: f32,
}

#[derive(Component)]
struct CShape {
    radius: f32,
    color: Color,
    vertices: f32,
}

#[derive(Component)]
struct CCollision {
    rad: f32,
}

#[derive(Component)]
struct CScore {
    score: f32,
}

#[derive(Component)]
struct CLifespan {
    remaining: f32,
    total: f32,
}

#[derive(Component, Default)]
struct CInput {
    up: bool,
    left: bool,
    right: bool,
    down: bool,
    shoot: Option<Vec2>,
}

#[derive(Component)]
struct Circle {
    name: String,
    position: Vec2,
    radius: f32,
    color: Color,
    vel: Vec2,
}

// endregion

// region: Tags

#[derive(Component)]
struct TEnemy;

#[derive(Component)]
struct TBullet;

#[derive(Component)]
struct TPlayer;

// endregion

// region: resources

#[derive(Resource)]
struct GameFontStyle(TextStyle);

#[derive(Resource)]
struct TimeSinceSpawn(f32);

// endregion

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::AutoVsync,
                    mode: WindowMode::Windowed,
                    position: WindowPosition::Automatic,
                    resolution: WindowResolution::new(800., 600.),
                    title: "very cool game".to_string(),
                    composite_alpha_mode: CompositeAlphaMode::Auto,
                    resizable: false,
                    enabled_buttons: EnabledButtons {
                        minimize: true,
                        maximize: false,
                        close: true,
                    },
                    decorations: true,
                    transparent: true,
                    focused: true,
                    window_level: WindowLevel::Normal,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ConfigPlugin,
            bevy_framepace::FramepacePlugin,
            EntropyPlugin::<WyRand>::default(),
        ))
        .add_state::<AppState>()
        .add_systems(OnEnter(AppState::InGame), (s_setup_window, s_setup_font))
        .add_systems(
            Update,
            (
                s_collisions,
                s_render,
                s_input,
                s_enemy_spawner,
                s_lifespan,
                s_movement.after(s_input),
            )
                .run_if(in_state(AppState::InGame)),
        )
        .insert_resource(TimeSinceSpawn(0.))
        .run();
}

// region: setup systems

fn s_setup_window(
    mut commands: Commands,
    mut framepace_settings: ResMut<bevy_framepace::FramepaceSettings>,
    mut windows: Query<&mut Window>,
    player_config: Res<PlayerConfig>,
    window_config: Res<WindowConfig>,
) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            viewport_origin: Vec2 { x: 0., y: 0. },
            ..Default::default()
        },
        ..Default::default()
    });

    let mut window = windows.single_mut();
    window
        .resolution
        .set(window_config.size.0, window_config.size.1);

    use bevy_framepace::Limiter;
    framepace_settings.limiter = Limiter::from_framerate(window_config.frame_limit.into());

    if window_config.fullscreen {
        window.mode = WindowMode::BorderlessFullscreen
    }

    let width = window.resolution.width();
    let height = window.resolution.height();

    spawn_player(&mut commands, &player_config, &width, &height)
}

fn s_setup_font(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    font_config: Res<FontConfig>,
) {
    commands.insert_resource(GameFontStyle(TextStyle {
        font: asset_server.load(font_config.file.clone()),
        font_size: font_config.size,
        color: Color::rgba(
            font_config.color.0,
            font_config.color.1,
            font_config.color.2,
            1.,
        ),
    }));
}

// endregion

// region: systems

fn s_render(
    circle_query: Query<(&CShape, &CTransform), Without<CLifespan>>,
    mut lifespan_query: Query<(&mut CShape, &CTransform, &CLifespan)>,
    mut gizmos: Gizmos,
) {
    for (shape, tf) in circle_query.iter() {
        gizmos
            .arc_2d(tf.pos, tf.angle, 2. * PI, shape.radius, shape.color)
            .segments(shape.vertices as usize);
    }

    for (mut shape, tf, ls) in lifespan_query.iter_mut() {
        shape.color.set_a(ls.remaining / ls.total);
        gizmos
            .arc_2d(tf.pos, tf.angle, 2. * PI, shape.radius, shape.color)
            .segments(shape.vertices as usize);
    }
}

fn s_movement(
    mut commands: Commands,
    mut circle_query: Query<(&CShape, &mut CTransform), Without<CInput>>,
    mut input_query: Query<(&CShape, &mut CInput, &mut CTransform)>,
    window: Query<&Window>,
    time: Res<Time>,
    player_config: Res<PlayerConfig>,
    bullet_config: Res<BulletConfig>,
) {
    let window = window.single();
    let width = window.resolution.width();
    let height = window.resolution.height();

    match input_query.get_single_mut() {
        Ok((shape, input, mut tf)) => {
            tf.vel = Vec2::ZERO;
            if input.up {
                tf.vel.y = 1.;
            }
            if input.down {
                tf.vel.y = -1.
            }
            if input.left {
                tf.vel.x = -1.
            }
            if input.right {
                tf.vel.x = 1.
            }

            if let Some(mut mouse_pos) = input.shoot {
                // convert from window coords to world space
                mouse_pos = Vec2::new(mouse_pos.x, (height - mouse_pos.y).abs());
                commands.spawn((
                    CShape {
                        radius: bullet_config.shape_radius,
                        color: Color::rgba(
                            bullet_config.color.0,
                            bullet_config.color.1,
                            bullet_config.color.2,
                            1.,
                        ),
                        vertices: bullet_config.vertices,
                    },
                    CTransform {
                        pos: tf.pos,
                        vel: (mouse_pos - tf.pos).normalize() * bullet_config.speed,
                        angle: 0.,
                    },
                    CLifespan {
                        remaining: bullet_config.lifespan,
                        total: bullet_config.lifespan,
                    },
                    CCollision {
                        rad: bullet_config.collision_radius,
                    },
                    TBullet,
                ));
            }

            tf.vel = tf.vel.normalize_or_zero();
            tf.vel = tf.vel * player_config.speed;
            transform_tick(
                tf.as_mut(),
                shape.radius,
                width,
                height,
                time.delta_seconds(),
            );
        }
        Err(err) => {
            error!("main.rs::movement_system: {}", err)
        }
    }

    for (shape, mut tf) in circle_query.iter_mut() {
        transform_tick(
            tf.as_mut(),
            shape.radius,
            width,
            height,
            time.delta_seconds(),
        );
    }
}

fn s_input(
    keyboard: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    mut input: Query<&mut CInput>,
    window: Query<&Window>,
) {
    match input.get_single_mut() {
        Ok(mut input) => {
            input.up = keyboard.pressed(KeyCode::W);
            input.down = keyboard.pressed(KeyCode::S);
            input.left = keyboard.pressed(KeyCode::A);
            input.right = keyboard.pressed(KeyCode::D);
            if mouse.just_pressed(MouseButton::Left) {
                input.shoot = window.single().cursor_position();
            } else {
                input.shoot = None;
            }
        }
        Err(err) => error!("main.rs::keyboard_system: {}", err),
    }
    if keyboard.pressed(KeyCode::Escape) {
        app_exit_events.send(bevy::app::AppExit)
    }
}

fn s_enemy_spawner(
    mut commands: Commands,
    time: Res<Time>,
    mut rng: ResMut<GlobalEntropy<WyRand>>,
    mut time_since_spawn: ResMut<TimeSinceSpawn>,
    enemy_config: Res<EnemyConfig>,
    windows: Query<&Window>,
) {
    time_since_spawn.0 += time.delta_seconds();
    if time_since_spawn.0 > enemy_config.spawn_interval {
        time_since_spawn.0 = 0.;
        let window = windows.single();
        commands.spawn((
            CTransform {
                pos: Vec2 {
                    x: rng_range(
                        rng.as_mut(),
                        enemy_config.shape_radius,
                        window.width() - enemy_config.shape_radius,
                    ),
                    y: rng_range(
                        rng.as_mut(),
                        enemy_config.shape_radius,
                        window.height() - enemy_config.shape_radius,
                    ),
                },
                vel: Vec2 {
                    x: rng_range(rng.as_mut(), enemy_config.min_speed, enemy_config.max_speed),
                    y: rng_range(rng.as_mut(), enemy_config.min_speed, enemy_config.max_speed),
                },
                angle: 0.,
            },
            CShape {
                radius: enemy_config.shape_radius,
                color: Color::rgba(
                    rng_range(rng.as_mut(), 0.2, 1.),
                    rng_range(rng.as_mut(), 0.2, 1.),
                    rng_range(rng.as_mut(), 0.2, 1.),
                    1.,
                ),
                vertices: rng_range_u32(
                    rng.as_mut(),
                    enemy_config.min_vertices as u32,
                    enemy_config.max_vertices as u32,
                ) as f32,
            },
            CCollision {
                rad: enemy_config.collision_radius,
            },
            TEnemy,
        ));
    }
}

fn s_collisions(
    mut commands: Commands,
    bullet_query: Query<(Entity, &CTransform, &CCollision, &TBullet)>,
    enemy_query: Query<(Entity, &CTransform, &CCollision, &TEnemy)>,
    mut player: Query<(Entity, &CTransform, &CCollision, &TPlayer)>,
    player_config: Res<PlayerConfig>,
    window: Query<&Window>,
) {
    let p = player.get_single_mut();
    for (e_e, e_tf, e_c, _) in enemy_query.iter() {
        if let Ok((p_e, p_tf, p_c, _)) = p {
            if is_collision(&e_tf.pos, &e_c.rad, &p_tf.pos, &p_c.rad) {
                commands.entity(p_e).despawn();
                let window = window.single();
                spawn_player(
                    &mut commands,
                    &player_config,
                    &window.width(),
                    &window.height(),
                );
                break;
            }
        }
        for (b_e, b_tf, b_c, _) in bullet_query.iter() {
            if is_collision(&e_tf.pos, &e_c.rad, &b_tf.pos, &b_c.rad) {
                commands.entity(e_e).despawn();
                commands.entity(b_e).despawn();
                break;
            }
        }
    }
}

fn s_lifespan(mut commands: Commands, mut query: Query<(Entity, &mut CLifespan)>, time: Res<Time>) {
    for (e, mut ls) in query.iter_mut() {
        ls.remaining -= time.delta_seconds();
        if ls.remaining < 0. {
            commands.entity(e).despawn();
        }
    }
}

// endregion

// region: functions

fn spawn_small_enemies(mut commands: Commands, tf: CTransform, s: CShape, small_lifespan: f32) {
    for i in 0..(s.vertices.round() as i8) {
        let i = f32::from(i);
        commands.spawn((
            CTransform {
                pos: tf.pos,
                vel: Vec2::from_angle(tf.angle * i),
                angle: tf.angle * i,
            },
            CShape {
                radius: s.radius / 2.,
                color: s.color,
                vertices: s.vertices,
            },
            CLifespan {
                remaining: todo!(),
                total: todo!(),
            },
        ));
    }
}

fn is_collision(pos1: &Vec2, rad1: &f32, pos2: &Vec2, rad2: &f32) -> bool {
    let diff = *pos1 - *pos2;
    let dist_sq = diff.x * diff.x + diff.y * diff.y;
    dist_sq < (rad1 + rad2) * (rad1 + rad2)
}

fn transform_tick(tf: &mut CTransform, radius: f32, width: f32, height: f32, delta_seconds: f32) {
    tf.pos = tf.pos + tf.vel;

    // wall collisions
    if tf.pos.x + radius > width || tf.pos.x - radius < 0. {
        tf.vel.x = -tf.vel.x;
        tf.pos.x = tf.pos.x + tf.vel.x * 2.;
    }
    if tf.pos.y + radius > height || tf.pos.y - radius < 0. {
        tf.vel.y = -tf.vel.y;
        tf.pos.y = tf.pos.y + tf.vel.y * 2.;
    }

    // rotation
    tf.angle += delta_seconds;
}

fn spawn_player(
    commands: &mut Commands,
    player_config: &Res<PlayerConfig>,
    width: &f32,
    height: &f32,
) {
    commands.spawn((
        CTransform {
            pos: Vec2 {
                x: width / 2.,
                y: height / 2.,
            },
            vel: Vec2 { x: 0., y: 0. },
            angle: 0.,
        },
        CShape {
            radius: player_config.shape_radius,
            color: Color::rgba(
                player_config.color.0,
                player_config.color.1,
                player_config.color.2,
                1.,
            ),
            vertices: player_config.vertices,
        },
        CCollision {
            rad: player_config.collision_radius,
        },
        CInput::default(),
        TPlayer,
    ));
}

fn rng_range(rng: &mut GlobalEntropy<WyRand>, min: f32, max: f32) -> f32 {
    min + (rng.next_u32() as f32) / ((u32::MAX as f32) / (max - min))
}

fn rng_range_u32(rng: &mut GlobalEntropy<WyRand>, min: u32, max: u32) -> u32 {
    min + (rng.next_u32() % (1 + max - min))
}

// endregion
