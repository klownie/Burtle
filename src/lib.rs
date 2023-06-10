use std::collections::VecDeque;

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution};
use bevy_prototype_lyon::prelude::*;

pub struct Burtle {}

impl Burtle {
    pub fn setup(width: f32, height: f32) {
        App::new()
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Burtle".into(),
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
            .add_system(burtle_movement)
            .run();
    }
}
#[derive(Component)]
pub struct Turtle {
    pub size: f32,
    pub heading: f32,
    pub pen_state: bool,
    pub pen_size: f32,
    pub pen_color: Color,
    pub registered_pos: (f32, Vec3),
    pub instruction: VecDeque<TurtleState>,
}

#[derive(Clone, Copy)]
pub enum TurtleState {
    PenUp,
    PenDown,
    TurnLeft(f32),
    TurnRight(f32),
    MoveForward(f32),
    MoveBackward(f32),
}

impl Turtle {
    pub fn right(&mut self, angle: f32) {
        self.instruction.push_back(TurtleState::TurnRight(angle))
    }
    pub fn left(&mut self, angle: f32) {
        self.instruction.push_back(TurtleState::TurnLeft(angle))
    }
    pub fn forward(&mut self, pixels: f32) {
        self.instruction.push_back(TurtleState::MoveForward(pixels))
    }
    pub fn backward(&mut self, pixels: f32) {
        self.instruction
            .push_back(TurtleState::MoveBackward(pixels))
    }
    pub fn pen_up(&mut self) {
        self.instruction.push_back(TurtleState::PenUp)
    }
    pub fn pen_down(&mut self) {
        self.instruction.push_back(TurtleState::PenDown)
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
            pen_state: false,
            pen_size: 2.,
            pen_color: Color::BLACK,
            registered_pos: (
                0.0,
                Vec3::new(window.width() / 2., window.height() / 2., 0.0),
            ),
            instruction: VecDeque::new(),
        },
    ));
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.0),
        ..default()
    });
}

fn burtle_movement(mut commands: Commands, mut turtle_query: Query<(&mut Transform, &mut Turtle)>) {
    for (mut transform, mut turtle) in turtle_query.iter_mut() {
        let Some(&instruction) = turtle.instruction.front() else {
            continue; 
        };
        match instruction {
            TurtleState::PenUp => {
                turtle.pen_state = false;
            }
            TurtleState::PenDown => {
                turtle.pen_state = true;
            }
            TurtleState::TurnLeft(angle) => {
                turtle.heading += angle;
            }
            TurtleState::TurnRight(angle) => {
                turtle.heading -= angle;
            }
            TurtleState::MoveForward(pixels) => {
                let direction = Vec3::new(
                    turtle.heading.to_radians().cos(),
                    turtle.heading.to_radians().sin(),
                    0.0,
                );
                if turtle.pen_state {
                    let old_pos = transform.translation.to_owned();
                    transform.translation -= direction * pixels;
                    let shape = shapes::Line(
                        Vec2::new(old_pos.x, old_pos.y),
                        Vec2::new(transform.translation.x, transform.translation.y),
                    );
                    commands.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&shape),
                            ..default()
                        },
                        Stroke::new(turtle.pen_color, turtle.pen_size),
                    ));
                } else {
                    transform.translation -= direction * pixels;
                }
            }
            TurtleState::MoveBackward(pixels) => {
                let direction = Vec3::new(
                    turtle.heading.to_radians().cos(),
                    turtle.heading.to_radians().sin(),
                    0.0,
                );

                if turtle.pen_state {
                    let old_pos = transform.translation.to_owned();
                    transform.translation -= direction * pixels;
                    let shape = shapes::Line(
                        Vec2::new(old_pos.x, old_pos.y),
                        Vec2::new(transform.translation.x, transform.translation.y),
                    );
                    commands.spawn((
                        ShapeBundle {
                            path: GeometryBuilder::build_as(&shape),
                            ..default()
                        },
                        Stroke::new(turtle.pen_color, turtle.pen_size),
                    ));
                } else {
                    transform.translation -= direction * pixels;
                }
            }
        }
        turtle.instruction.pop_front();
    }
}
