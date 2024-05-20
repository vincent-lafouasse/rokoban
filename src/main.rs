#![allow(dead_code)]

use bevy::prelude::*;

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Box;

#[derive(Component)]
struct Hole;

fn init_world(mut commands: Commands) {
    commands.spawn((Player, Position { x: 0, y: 0 }));
    commands.spawn((Box, Position { x: 1, y: 0 }));
    commands.spawn((Box, Position { x: 4, y: 0 }));
    commands.spawn((Hole, Position { x: 2, y: 0 }));
}

fn say_hi() {
    println!("hi");
}

fn log_player(query: Query<&Position, With<Player>>) {
    for pos in &query {
        println!("the player is at the position x:{} y:{}", pos.x, pos.y);
    }
}

fn log_boxes(query: Query<&Position, With<Box>>) {
    for pos in &query {
        println!("there is a box at the position x:{} y:{}", pos.x, pos.y);
    }
}

fn main() {
    App::new()
        .add_systems(Startup, init_world)
        .add_systems(Update, (say_hi, log_player, log_boxes))
        .run();
}
