use bevy::{prelude::*, window::*};

// region: Components

#[derive(Component)]
struct Circle {
    name: String,
    position: Vec2,
    radius: f32,
    color: Color,
    vel: Vec2,
}

#[derive(Component)]
struct Rectangle {
    name: String,
    position: Vec2,
    size: Vec2,
    color: Color,
    vel: Vec2,
}

// endregion

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync,
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
        }))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (draw_shapes_system, move_shapes_system, keyboard_system),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            viewport_origin: Vec2 { x: 0., y: 0. },
            ..Default::default()
        },
        ..Default::default()
    });
    // text
    commands.spawn(TextBundle::from_section(
        "Hold 'Left' or 'Right' to change the line width",
        TextStyle {
            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
            font_size: 28.,
            color: Color::WHITE,
        },
    ));

    let circles = vec![
        Circle {
            name: String::from("CGreen"),
            position: Vec2 { x: 100., y: 100. },
            vel: Vec2 { x: -0.03, y: 0.02 },
            color: Color::Rgba {
                red: 0.,
                green: 255.,
                blue: 0.,
                alpha: 255.,
            },
            radius: 50.,
        },
        Circle {
            name: String::from("CBlue"),
            position: Vec2 { x: 200., y: 200. },
            vel: Vec2 { x: 0.02, y: 0.04 },
            color: Color::Rgba {
                red: 0.,
                green: 0.,
                blue: 255.,
                alpha: 255.,
            },
            radius: 100.,
        },
        Circle {
            name: String::from("CPurple"),
            position: Vec2 { x: 300., y: 300. },
            vel: Vec2 { x: -0.02, y: -0.01 },
            color: Color::Rgba {
                red: 255.,
                green: 0.,
                blue: 255.,
                alpha: 255.,
            },
            radius: 75.,
        },
    ];
    for shape in circles {
        commands.spawn(shape);
    }
    let rectangles = vec![
        Rectangle {
            name: String::from("RRed"),
            position: Vec2 { x: 200., y: 200. },
            vel: Vec2 { x: 0.1, y: 0.15 },
            color: Color::Rgba {
                red: 255.,
                green: 0.,
                blue: 0.,
                alpha: 255.,
            },
            size: Vec2 { x: 50., y: 25. },
        },
        Rectangle {
            name: String::from("RGrey"),
            position: Vec2 { x: 300., y: 250. },
            vel: Vec2 { x: -0.02, y: 0.02 },
            color: Color::Rgba {
                red: 100.,
                green: 100.,
                blue: 100.,
                alpha: 255.,
            },
            size: Vec2 { x: 50., y: 100. },
        },
        Rectangle {
            name: String::from("RTeal"),
            position: Vec2 { x: 125., y: 100. },
            vel: Vec2 { x: -0.02, y: 0.02 },
            color: Color::Rgba {
                red: 0.,
                green: 255.,
                blue: 255.,
                alpha: 255.,
            },
            size: Vec2 { x: 100., y: 100. },
        },
    ];
    for shape in rectangles {
        commands.spawn(shape);
    }
}

fn draw_shapes_system(
    text_query: Query<(Entity, &Text)>,
    circle_query: Query<(Entity, &Circle)>,
    rect_query: Query<(Entity, &Rectangle)>,
    mut gizmos: Gizmos,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (entity, _) in text_query.iter() {
        commands.entity(entity).despawn();
    }
    for (_, circle) in circle_query.iter() {
        gizmos.circle_2d(circle.position, circle.radius, circle.color);
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                circle.name.clone(),
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 12.,
                    color: Color::BLACK,
                },
            )
            .with_alignment(TextAlignment::Center),
            transform: Transform {
                translation: Vec3::new(circle.position.x, circle.position.y, 0.),
                ..Default::default()
            },
            ..Default::default()
        });
    }
    for (_, rect) in rect_query.iter() {
        gizmos.rect_2d(rect.position, 0., rect.size, rect.color);
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                rect.name.clone(),
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 12.,
                    color: Color::BLACK,
                },
            )
            .with_alignment(TextAlignment::Center),
            transform: Transform {
                translation: Vec3::new(rect.position.x, rect.position.y, 0.),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

fn move_shapes_system(
    mut circle_query: Query<&mut Circle>,
    mut rect_query: Query<&mut Rectangle>,
    window: Query<&Window>,
) {
    let window = window.single();

    let width = window.resolution.width();
    let height = window.resolution.height();

    for mut circle in circle_query.iter_mut() {
        let vel = circle.vel;
        let pos = circle.position;

        if pos.x + circle.radius > width {
            circle.vel.x = -vel.x;
        } else if pos.x - circle.radius < 0. {
            circle.vel.x = -vel.x;
        }
        if pos.y + circle.radius > height {
            circle.vel.y = -vel.y
        } else if pos.y - circle.radius < 0. {
            circle.vel.y = -vel.y
        }

        let v = circle.vel;
        circle.position += v;
    }
    for mut rect in rect_query.iter_mut() {
        let vel = rect.vel;
        let pos = rect.position;
        let size = rect.size;

        if pos.x + size.x / 2. > width {
            rect.vel.x = -vel.x;
        } else if pos.x - size.x / 2. < 0. {
            rect.vel.x = -vel.x;
        }

        if pos.y + size.y / 2. > height {
            rect.vel.y = -vel.y;
        } else if pos.y - size.y / 2. < 0. {
            rect.vel.y = -vel.y;
        }

        let v = rect.vel;
        rect.position += v;
    }
}

fn keyboard_system(
    keyboard: Res<Input<KeyCode>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    if keyboard.pressed(KeyCode::Escape) {
        app_exit_events.send(bevy::app::AppExit)
    }
}
