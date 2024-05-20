#![allow(dead_code)]

use bevy::prelude::*;

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
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
        pos: Position { x: 0, y: 0 },
    });
    commands.spawn(Box {
        pos: Position { x: 1, y: 0 },
    });
    commands.spawn(Hole {
        pos: Position { x: 2, y: 0 },
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
