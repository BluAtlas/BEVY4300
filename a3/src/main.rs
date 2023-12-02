// region: includes

use std::time::Duration;

use bevy::{prelude::*, window::*};
use bevy_rand::{plugin::EntropyPlugin, prelude::WyRand, resource::GlobalEntropy};
use config::*;
use leafwing_input_manager::prelude::*;
use rand_core::RngCore;

mod config;

// endregion

// region: AppState

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Loading,
    PostLoading,
    InGame,
}

// endregion

// region: Components

#[derive(Component)]
struct CTransform {
    pos: Vec2,
    prev_pos: Vec2,
    scale: Vec2,
    vel: Vec2,
    angle: f32,
}

impl CTransform {
    fn new(pos: Vec2) -> Self {
        Self {
            pos,
            prev_pos: pos,
            scale: Vec2::ONE,
            vel: Vec2::ZERO,
            angle: 0.,
        }
    }
    fn new_moving(pos: Vec2, vel: Vec2, angle: f32) -> Self {
        Self {
            pos,
            prev_pos: pos,
            scale: Vec2::ONE,
            vel,
            angle,
        }
    }
}

#[derive(Component)]
struct CBoundingBox {
    size: Vec2,
    half_size: Vec2,
    color: Color,
}

impl CBoundingBox {
    pub fn new(size: Vec2) -> Self {
        Self {
            size: size,
            half_size: Vec2 {
                x: size.x / 2.,
                y: size.y / 2.,
            },
            color: Color::rgba(1., 1., 1., 0.),
        }
    }
    pub fn new_c(size: Vec2, color: Color) -> Self {
        Self {
            size: size,
            half_size: Vec2 {
                x: size.x / 2.,
                y: size.y / 2.,
            },
            color: color,
        }
    }
}

#[derive(Component)]
struct CLifespan {
    remaining: f32,
    total: f32,
}

impl CLifespan {
    fn new(total: f32) -> Self {
        Self {
            remaining: total,
            total,
        }
    }
}

#[derive(Component)]
struct CGravity(f32);

#[derive(Component)]
enum CState {
    Jumping,
    Standing,
    WalkLeft,
    WalkRight,
}

#[derive(Component)]
struct CAnimated {
    anims: Animations,
    timer: Option<Timer>,
}

impl CAnimated {
    fn new(anims: Animations, ms_per_frame: u64) -> Self {
        Self {
            anims,
            timer: Some(Timer::new(
                Duration::from_millis(ms_per_frame),
                TimerMode::Repeating,
            )),
        }
    }
}

#[derive(Clone)]
struct Animations {
    idle: Animation,
    walk: Option<Animation>,
    jump: Option<Animation>,
}

#[derive(Clone)]
struct Animation {
    anim: Handle<TextureAtlas>,
    len: usize,
    ms_per_frame: u64,
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum Action {
    Left,
    Right,
    Up,
    Down,
    Jump,
    Shoot,
    Quit,
}

// endregion

// region: Tags

#[derive(Component)]
struct TTile;

#[derive(Component)]
struct TDecoration;

#[derive(Component)]
struct TPlayer;

// endregion

// region: Resources

#[derive(Resource)]
struct GameAnimations {
    blue_slime: Animations,
}

// endregion

// region: Bundles

#[derive(Bundle)]
struct PlayerBundle {
    tf: CTransform,
    bb: CBoundingBox,
    anim: CAnimated,
    sprite: SpriteSheetBundle,
    tag: TPlayer,
}

#[derive(Bundle)]
struct TileBundle {
    tf: CTransform,
    bb: CBoundingBox,
    //anim: CAnimated,
    //sprite: SpriteSheetBundle,
    tag: TTile,
}

#[derive(Bundle)]
struct DecorationBundle {
    tf: CTransform,
    // TODO DELETE DECORATIONS HAVING BOUNDING BOXES, REPLACE WITH DRAWING SPRITES
    bb: CBoundingBox,
    tag: TDecoration,
}

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
                    title: "very cool game 2".to_string(),
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
            InputManagerPlugin::<Action>::default(),
        ))
        .add_state::<AppState>()
        .add_systems(
            OnEnter(AppState::InGame),
            (
                s_setup_window,
                s_setup_input,
                s_setup_level,
                s_setup_textures,
            ),
        )
        .add_systems(
            Update,
            (
                s_lifespan,
                s_movement,
                s_collision.after(s_movement),
                s_match_anims_to_transforms.after(s_collision),
                s_animation.after(s_collision),
                s_render.after(s_collision),
            )
                .run_if(in_state(AppState::InGame)),
        )
        .run();
}

// region: startup systems

fn s_setup_textures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    animation_config: Res<AnimationConfig>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    player_config: Res<PlayerConfig>,
) {
    let game_animations = GameAnimations {
        blue_slime: Animations {
            idle: Animation {
                anim: texture_atlases.add(TextureAtlas::from_grid(
                    asset_server.load(animation_config.blue_slime_idle.0.clone()),
                    Vec2::new(
                        animation_config.blue_slime_idle.1,
                        animation_config.blue_slime_idle.2,
                    ),
                    animation_config.blue_slime_idle.3,
                    animation_config.blue_slime_idle.4,
                    None,
                    None,
                )),
                len: (animation_config.blue_slime_idle.3 * animation_config.blue_slime_idle.4),
                ms_per_frame: animation_config.blue_slime_idle.5,
            },
            walk: None,
            jump: None,
        },
    };

    spawn_player(&mut commands, &player_config, &game_animations);

    commands.insert_resource(game_animations);
}

// endregion

// region: setup systems

fn s_setup_window(
    mut commands: Commands,
    mut framepace_settings: ResMut<bevy_framepace::FramepaceSettings>,
    mut windows: Query<&mut Window>,
    window_config: Res<WindowConfig>,
) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            viewport_origin: Vec2 { x: 0., y: 0. },
            near: -1000.,
            far: 1000.,
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
}

fn s_setup_input(mut commands: Commands) {
    commands.spawn(InputManagerBundle::<Action> {
        // Stores "which actions are currently pressed"
        action_state: ActionState::default(),
        // Describes how to convert from player inputs into those actions
        input_map: InputMap::new([
            (KeyCode::W, Action::Jump),
            (KeyCode::W, Action::Up),
            (KeyCode::S, Action::Down),
            (KeyCode::A, Action::Left),
            (KeyCode::D, Action::Right),
            (KeyCode::Space, Action::Shoot),
            (KeyCode::Escape, Action::Quit),
        ]),
    });
}

fn s_setup_level(mut commands: Commands, level_config: Res<LevelConfig>) {
    for (solid, kind, grid_x, grid_y) in level_config.tiles.iter() {
        let grid_x = (*grid_x * 64.) + 32.;
        let grid_y = (*grid_y * 64.) + 32.;
        if *solid == 1 {
            commands.spawn(TileBundle {
                tf: CTransform::new(Vec2::new(grid_x, grid_y)),
                bb: CBoundingBox::new_c(Vec2::new(64., 64.), Color::BLUE),
                tag: TTile,
            });
        } else {
            commands.spawn(DecorationBundle {
                tf: CTransform::new(Vec2::new(grid_x, grid_y)),
                bb: CBoundingBox::new_c(Vec2::new(64., 64.), Color::BLUE),
                tag: TDecoration,
            });
        }
    }
}

// endregion

// region: systems

fn s_render(circle_query: Query<(&CBoundingBox, &CTransform)>, mut gizmos: Gizmos) {
    for (bb, tf) in circle_query.iter() {
        gizmos.rect_2d(tf.pos, tf.angle, bb.size, bb.color)
    }
}

fn s_movement(
    input_query: Query<&ActionState<Action>>,
    mut player_query: Query<(&CBoundingBox, &mut CTransform, &TPlayer)>,
    window: Query<&Window>,
    player_config: Res<PlayerConfig>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    let window = window.single();
    let width = window.resolution.width();
    let height = window.resolution.height();

    let actions = input_query.single();

    match player_query.get_single_mut() {
        Ok((bb, mut tf, _)) => {
            tf.vel = Vec2::ZERO;
            if actions.pressed(Action::Up) {
                tf.vel.y = 1.;
            }
            if actions.pressed(Action::Down) {
                tf.vel.y = -1.
            }
            if actions.pressed(Action::Left) {
                tf.vel.x = -1.
            }
            if actions.pressed(Action::Right) {
                tf.vel.x = 1.
            }
            if actions.pressed(Action::Quit) {
                app_exit_events.send(bevy::app::AppExit)
            }

            tf.vel = tf.vel.normalize_or_zero();
            tf.vel = tf.vel * player_config.walk_speed;

            tf.prev_pos = tf.pos;
            tf.pos = tf.pos + tf.vel;
        }
        Err(err) => {
            error!("main.rs::movement_system: {}", err)
        }
    }
}

fn s_collision(
    query: Query<(&CBoundingBox, &CTransform), (With<TTile>, Without<TPlayer>)>,
    mut player_query: Query<(&CBoundingBox, &mut CTransform), With<TPlayer>>,
) {
    if let Ok((p_bb, mut p_tf)) = player_query.get_single_mut() {
        for (t_bb, t_tf) in query.iter() {
            // Calculating collisions using Axis Aligned Bounding Boxes
            let overlap = get_bounding_overlap(&p_bb.size, &p_tf.pos, &t_bb.size, &t_tf.pos);
            if overlap.y > 0. && overlap.x > 0. {
                let prev_overlap =
                    get_bounding_overlap(&p_bb.size, &p_tf.prev_pos, &t_bb.size, &t_tf.prev_pos);
                resolve_collision_by_moving(&overlap, &prev_overlap, &mut p_tf.pos, &t_tf.pos)
            }
        }
    };
}

fn s_lifespan(mut commands: Commands, mut query: Query<(Entity, &mut CLifespan)>, time: Res<Time>) {
    for (e, mut ls) in query.iter_mut() {
        ls.remaining -= time.delta_seconds();
        if ls.remaining < 0. {
            commands.entity(e).despawn();
        }
    }
}

fn s_animation(mut query: Query<(&mut CAnimated, &mut TextureAtlasSprite)>, time: Res<Time>) {
    for (mut anim, mut sprite) in query.iter_mut() {
        if let Some(timer) = anim.timer.as_mut() {
            timer.tick(time.delta());
            if timer.just_finished() {
                sprite.index = if sprite.index == anim.anims.idle.len - 1 {
                    0
                } else {
                    sprite.index + 1
                };
            }
        } else {
        }
    }
}

// Eventually replace CTransform with bevy's Transform
fn s_match_anims_to_transforms(mut query: Query<(&CTransform, &mut Transform)>) {
    for (tf, mut tr) in query.iter_mut() {
        tr.translation = Vec3::new(tf.pos.x, tf.pos.y, 0.)
    }
}

// endregion

// region: functions

fn spawn_player(
    commands: &mut Commands,
    player_config: &Res<PlayerConfig>,
    game_animations: &GameAnimations,
) {
    let x = (player_config.starting_position.0 * 64.) + 32.;
    let y = (player_config.starting_position.1 * 64.) + player_config.bounding_box.1 / 2.;

    commands.spawn(PlayerBundle {
        tf: CTransform::new(Vec2::new(x, y)),
        bb: CBoundingBox::new_c(
            Vec2::new(player_config.bounding_box.0, player_config.bounding_box.1),
            Color::RED,
        ),
        anim: CAnimated::new(
            game_animations.blue_slime.clone(),
            game_animations.blue_slime.idle.ms_per_frame,
        ),
        tag: TPlayer,
        sprite: SpriteSheetBundle {
            texture_atlas: game_animations.blue_slime.idle.anim.clone(),
            transform: Transform::from_xyz(x, y, 0.),
            ..Default::default()
        },
    });
}

fn get_bounding_overlap(p_size: &Vec2, p_pos: &Vec2, t_size: &Vec2, t_pos: &Vec2) -> Vec2 {
    let diff = Vec2::new((p_pos.x - t_pos.x).abs(), (p_pos.y - t_pos.y).abs());
    let overlap_x = (p_size.x / 2.) + (t_size.x / 2.) - diff.x;
    let overlap_y = (p_size.y / 2.) + (t_size.y / 2.) - diff.y;
    Vec2::new(overlap_x, overlap_y)
}

fn resolve_collision_by_moving(
    overlap: &Vec2,
    prev_overlap: &Vec2,
    p_pos: &mut Vec2,
    t_pos: &Vec2,
) {
    if prev_overlap.x > 0. {
        if p_pos.y > t_pos.y {
            // Vertical Collision from above
            p_pos.y += overlap.y;
        } else {
            // Vertical Collision from below
            p_pos.y -= overlap.y;
        }
    } else if prev_overlap.y > 0. {
        if p_pos.x > t_pos.x {
            // Horizontal Collision from right
            p_pos.x += overlap.x;
        } else {
            // Horizontal Collision from left
            p_pos.x -= overlap.x;
        }
    } else {
        if p_pos.y > t_pos.y && p_pos.x < t_pos.y {
            // Collided from Top Left, Move Up
            p_pos.y += overlap.y;
        }
        if p_pos.y > t_pos.y && p_pos.x > t_pos.y {
            // Collided from Top Right, Move Up
            p_pos.y += overlap.y;
        }
        if p_pos.y < t_pos.y && p_pos.x < t_pos.y {
            // Collided from Bottom Left, Move Left
            p_pos.x -= overlap.x;
        }
        if p_pos.y < t_pos.y && p_pos.x > t_pos.y {
            // Collided from Bottom Right, Move Right
            p_pos.x += overlap.x;
        }
    }
}

fn rng_range(rng: &mut GlobalEntropy<WyRand>, min: f32, max: f32) -> f32 {
    min + (rng.next_u32() as f32) / ((u32::MAX as f32) / (max - min))
}

fn rng_range_u32(rng: &mut GlobalEntropy<WyRand>, min: u32, max: u32) -> u32 {
    min + (rng.next_u32() % (1 + max - min))
}

// endregion
