use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_prototype_lyon::prelude::*;

pub struct Burtle {}

impl Burtle {
    pub fn new() {
        App::new()
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "FLOAT".into(),
                    resolution: (1920., 1080.).into(),
                    present_mode: PresentMode::AutoVsync,
                    // Tells wasm to resize the window according to the available canvas
                    fit_canvas_to_parent: false,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
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
    horizontale: f32,
    pen_state: bool,
    registered_pos: (f32, Vec3),
    instruction: String,
}

impl Turtle {
    pub fn right(&mut self, angle: f32) {
        self.horizontale += angle
    }
    pub fn left(&mut self, angle: f32) {
        self.horizontale -= angle
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                flip_x: false,
                flip_y: false,
                custom_size: Some(Vec2::new(100.0, 100.0)),
                anchor: Default::default(),
                ..default()
            },
/*             transform: Transform::from_xyz(window.width() / 3.0, window.height() / 2.0, 0.0), */
            transform: Transform::from_xyz(0.0, 0.0 , 0.0),
            texture: asset_server.load("sprites/turtle.png"),
            ..default()
        },
        Turtle {
            horizontale: 0.0,
            pen_state: false,
            registered_pos: (
                0.0,
                Vec3::new( 0.0, 0.0, 0.0),
            ),
            instruction: String::new(),
        },
    ));
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}
