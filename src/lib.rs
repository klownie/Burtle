use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution};
use bevy_prototype_lyon::prelude::*;

pub struct Burtle {}

impl Burtle {
    pub fn setup(width: f32, height: f32) {
        App::new()
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "FLOAT".into(),
                    resolution:
                        WindowResolution::new(width, height).with_scale_factor_override(1.0),
                    present_mode: PresentMode::AutoVsync,

                    /*  window_theme: Some(WindowTheme::Dark), */
                    ..default()
                }),
                ..default()
            }))
            .add_plugin(ShapePlugin)
            .add_startup_system(setup)
            .run();
    }
}
#[derive(Component)]
pub struct Turtle {
    pub size: f32,
    pub heading: f32,
    pub speed: f32,
    pub pen_state: bool,
    pub registered_pos: (f32, Vec3),
    pub instruction: String,
}

impl Turtle {
    pub fn right(&mut self, angle: f32) {
        self.heading += angle
    }
    pub fn left(&mut self, angle: f32) {
        self.heading -= angle
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, windows: Query<&Window>) {
    let window = windows.single();
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                flip_x: false,
                flip_y: false,
                custom_size: Some(Vec2::new(100.0, 100.0)),
                anchor: Default::default(),
                ..default()
            },
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.0),
            texture: asset_server.load("sprites/turtle.png"),
            ..default()
        },
        Turtle {
            size: 100.,
            heading: 0.,
            speed: 0.,
            pen_state: false,
            registered_pos: (
                0.0,
                Vec3::new(window.width() / 2., window.height() / 2., 0.0),
            ),
            instruction: String::new(),
        },
    ));
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.0),
        ..default()
    });
}
