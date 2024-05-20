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
    // spawn entities
    commands.spawn((Player, Position { x: 0, y: 0 }));
    commands.spawn((Box, Position { x: 1, y: 0 }));
    commands.spawn((Box, Position { x: 4, y: 0 }));
    commands.spawn((Hole, Position { x: 2, y: 0 }));
}

fn say_hi() {
    println!("hi");
}

// reads as "iterate over every `Position` component for Entities that also have a `Player`
// component"
fn log_player(query: Query<&Position, With<Player>>) {
    for pos in &query {
        println!("the player is at the position x:{} y:{}", pos.x, pos.y);
    }
}

// same but mutably
fn move_player_left(mut query: Query<&mut Position, With<Player>>) {
    for mut pos in &mut query {
        pos.x += 1;
    }
}

fn log_boxes(query: Query<&Position, With<Box>>) {
    for pos in &query {
        println!("there is a box at the position x:{} y:{}", pos.x, pos.y);
    }
}

struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_world).add_systems(
            // if not chained, those systems are run in parallel
            Update,
            (say_hi, (log_player, move_player_left, log_player).chain()),
        );
    }
}

fn main() {
    App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
}
