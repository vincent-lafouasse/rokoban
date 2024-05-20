#![allow(dead_code)]

use bevy::prelude::*;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Player {
    pos: Position,
}

#[derive(Component)]
struct Box {
    pos: Position,
}

#[derive(Component)]
struct Hole {
    pos: Position,
}

fn init_world(mut commands: Commands) {
    commands.spawn(Player {
        pos: Position { x: 0.0, y: 0.0 },
    });
}

fn say_hi() {
    println!("hi");
}

fn main() {
    App::new()
        .add_systems(Startup, init_world)
        .add_systems(Update, say_hi)
        .run();
}
