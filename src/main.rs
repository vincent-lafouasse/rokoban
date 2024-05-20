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

#[derive(Resource)]
struct GreetTimer(Timer);

// reads as "iterate over every `Position` component for Entities that also have a `Player`
// component"
fn log_player(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Position, With<Player>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for pos in &query {
            println!("the player is at the position x:{} y:{}", pos.x, pos.y);
        }
    }
}

// same but mutably
fn move_player_left(mut query: Query<&mut Position, With<Player>>) {
    for mut pos in &mut query {
        pos.x += 1;
    }
}

struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, init_world)
            .add_systems(
                // if not chained, those systems are run in parallel
                Update, log_player,
            );
    }
}

fn main() {
    App::new().add_plugins((DefaultPlugins, HelloPlugin)).run();
}
