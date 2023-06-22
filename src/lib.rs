pub use bevy::prelude::Color;
use bevy::prelude::*;
use bevy::window::close_on_esc;
use bevy::window::{PresentMode, WindowResolution};
use bevy_despawn_with::DespawnAllCommandsExt;
use bevy_prototype_lyon::prelude::*;
use std::borrow::BorrowMut;
use std::collections::VecDeque;

impl Burtle {
    pub fn run(self, width: f32, height: f32) {
        App::new()
            .insert_resource(BurtleInstruction(self.instruction))
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Burtle".into(),
                    resolution:
                        WindowResolution::new(width, height).with_scale_factor_override(1.0),
                    present_mode: PresentMode::AutoVsync,
                    resizable: false,
                    ..default()
                }),
                ..default()
            }))
            .add_plugin(ShapePlugin)
            .add_startup_system(setup)
            .add_systems((burtle_movement, close_on_esc))
            .run()
    }
}

#[derive(Resource)]
pub struct BurtleInstruction(VecDeque<BurtleCommand>);

#[derive(Component)]
pub struct Burtle {
    heading: f32,
    #[cfg(feature = "lindenmayer")]
    waypoint_coord: VecDeque<Vec3>,
    #[cfg(feature = "lindenmayer")]
    waypoint_heading: VecDeque<f32>,
    pen_state: bool,
    pen_size: f32,
    pen_color: Color,
    instruction: VecDeque<BurtleCommand>,
}

#[derive(Clone, Copy, Debug)]
pub enum BurtleCommand {
    PenUp,
    PenDown,
    TurnLeft(f32),
    TurnRight(f32),
    MoveForward(f32),
    MoveBackward(f32),
    SetPenColor(Color),
    SetPenSize(f32),
    SetSize(f32),
    GoTo(f32, f32),
    SetHeading(f32),
    Wait(u32),
    Clear,
    #[cfg(feature = "lindenmayer")]
    AddWaypoint,
    #[cfg(feature = "lindenmayer")]
    RestoreWaypoint,
}

impl Default for Burtle {
    fn default() -> Self {
        Self {
            heading: 0.,
            #[cfg(feature = "lindenmayer")]
            waypoint_coord: VecDeque::new(),
            #[cfg(feature = "lindenmayer")]
            waypoint_heading: VecDeque::new(),
            pen_state: false,
            pen_size: 2.,
            pen_color: Color::BLACK,
            instruction: VecDeque::new(),
        }
    }
}

// impl Drop for Burtle {
//     fn drop(&mut self) {
//         self.run(1000., 1000.);
//     }
// }

impl Burtle {
    pub fn new() -> Burtle {
        Burtle { ..default() }
    }
    pub fn right(&mut self, angle: f32) {
        self.instruction.push_back(BurtleCommand::TurnRight(angle))
    }
    pub fn left(&mut self, angle: f32) {
        self.instruction.push_back(BurtleCommand::TurnLeft(angle))
    }
    pub fn forward(&mut self, pixels: f32) {
        self.instruction
            .push_back(BurtleCommand::MoveForward(pixels))
    }
    pub fn backward(&mut self, pixels: f32) {
        self.instruction
            .push_back(BurtleCommand::MoveBackward(pixels))
    }
    pub fn pen_up(&mut self) {
        self.instruction.push_back(BurtleCommand::PenUp)
    }
    pub fn pen_down(&mut self) {
        self.instruction.push_back(BurtleCommand::PenDown)
    }
    pub fn set_pen_color(&mut self, color: Color) {
        self.instruction
            .push_back(BurtleCommand::SetPenColor(color))
    }
    pub fn set_pen_size(&mut self, size: f32) {
        self.instruction.push_back(BurtleCommand::SetPenSize(size))
    }
    pub fn goto(&mut self, x: f32, y: f32) {
        self.instruction.push_back(BurtleCommand::GoTo(x, y))
    }
    pub fn set_heading(&mut self, direction: f32) {
        self.instruction
            .push_back(BurtleCommand::SetHeading(direction))
    }
    pub fn wait(&mut self, frames: u32) {
        self.instruction.push_back(BurtleCommand::Wait(frames))
    }
    pub fn clear(&mut self) {
        self.instruction.push_back(BurtleCommand::Clear)
    }
    pub fn reset(&mut self) {
        self.instruction.push_back(BurtleCommand::GoTo(0., 0.));
        self.instruction.push_back(BurtleCommand::Clear)
    }
    #[cfg(feature = "lindenmayer")]
    pub fn set_waypoint(&mut self) {
        self.instruction.push_back(BurtleCommand::AddWaypoint)
    }
    #[cfg(feature = "lindenmayer")]
    pub fn goto_waypoint(&mut self) {
        self.instruction.push_back(BurtleCommand::RestoreWaypoint)
    }
}

fn setup(instructions: Res<BurtleInstruction>, mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                flip_x: false,
                flip_y: false,
                color: Color::DARK_GREEN,
                custom_size: Some(Vec2::new(10.0, 10.0)),
                anchor: Default::default(),
                ..default()
            },
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Burtle {
            instruction: instructions.0.clone(),
            ..default()
        },
    ));
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });
}

fn burtle_movement(
    mut commands: Commands,
    mut turtle_query: Query<(&mut Transform, &mut Burtle, &mut Sprite)>,
) {
    for (mut transform, mut turtle, mut sprite) in turtle_query.iter_mut() {
        for _ in 0..turtle.instruction.len() {
            let Some(&instruction) = turtle.instruction.front() else {
            continue; 
        };
            match instruction {
                BurtleCommand::PenUp => {
                    turtle.pen_state = false;
                }
                BurtleCommand::PenDown => {
                    turtle.pen_state = true;
                }
                BurtleCommand::TurnLeft(angle) => {
                    turtle.heading += angle;
                }
                BurtleCommand::TurnRight(angle) => {
                    turtle.heading -= angle;
                }
                BurtleCommand::MoveForward(pixels) => {
                    let direction = Vec3::new(
                        turtle.heading.to_radians().cos(),
                        turtle.heading.to_radians().sin(),
                        0.0,
                    );
                    if turtle.pen_state {
                        draw_line(
                            turtle.as_mut(),
                            transform.as_mut(),
                            commands.borrow_mut(),
                            direction,
                            pixels,
                        )
                    } else {
                        transform.translation -= direction * pixels;
                    }
                }
                BurtleCommand::MoveBackward(pixels) => {
                    let direction = Vec3::new(
                        turtle.heading.to_radians().cos(),
                        turtle.heading.to_radians().sin(),
                        0.0,
                    );

                    if turtle.pen_state {
                        draw_line(
                            turtle.as_mut(),
                            transform.as_mut(),
                            commands.borrow_mut(),
                            direction,
                            pixels,
                        )
                    } else {
                        transform.translation -= direction * pixels;
                    }
                }
                BurtleCommand::SetPenColor(color) => turtle.pen_color = color,
                BurtleCommand::SetPenSize(size) => turtle.pen_size = size,
                BurtleCommand::SetSize(size) => sprite.custom_size = Some(Vec2::new(size, size)),
                BurtleCommand::GoTo(x, y) => {
                    if turtle.pen_state {
                        let old_pos = transform.translation.to_owned();
                        transform.translation = Vec3::new(x, y, 0.);
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
                        transform.translation = Vec3::new(x, y, 0.);
                    }
                }
                BurtleCommand::SetHeading(direction) => turtle.heading = direction,
                BurtleCommand::Wait(mut frames) => {
                    if frames == 0 {
                        turtle.instruction.pop_front();
                        continue;
                    }
                    frames -= 1;
                    turtle.instruction.pop_front();
                    turtle.instruction.push_front(BurtleCommand::Wait(frames));
                    break;
                }
                BurtleCommand::Clear => commands.despawn_all::<With<Stroke>>(),
                #[cfg(feature = "lindenmayer")]
                BurtleCommand::AddWaypoint => {
                    turtle.instruction.pop_front();
                    turtle
                        .waypoint_coord
                        .push_front(transform.translation.to_owned());
                    let heading = turtle.heading.to_owned();
                    turtle.waypoint_heading.push_front(heading);
                    continue;
                }
                #[cfg(feature = "lindenmayer")]
                BurtleCommand::RestoreWaypoint => {
                    turtle.instruction.pop_front();
                    let heading = turtle.waypoint_heading.pop_front();
                    let coord = turtle.waypoint_coord.pop_front();
                    if let (Some(heading), Some(coord)) = (heading, coord) {
                        turtle
                            .instruction
                            .push_front(BurtleCommand::GoTo(coord.x, coord.y));
                        turtle
                            .instruction
                            .push_front(BurtleCommand::SetHeading(heading))
                    } else {
                        turtle.instruction.push_front(BurtleCommand::GoTo(0., 0.));
                        turtle.instruction.push_front(BurtleCommand::SetHeading(0.))
                    }
                    continue;
                }
            }
            turtle.instruction.pop_front();
        }
    }

    fn draw_line(
        turtle: &mut Burtle,
        transform: &mut Transform,
        commands: &mut Commands,
        direction: Vec3,
        length: f32,
    ) {
        let old_pos = transform.translation.to_owned();
        transform.translation -= direction * length;
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
    }
}
