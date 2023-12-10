use crate::{app_state::*, level::*, player::spawn_player};
use bevy::prelude::*;

pub struct Level1Plugin;

impl Plugin for Level1Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_level)
            .add_systems(OnExit(GameState::GameOver), despawn_level)
            .add_systems(OnExit(AppState::InGame), despawn_level);
    }
}

#[derive(Component)]
pub struct Level1;

fn spawn_level(mut commands: Commands) {
    // Ground
    commands.spawn(LevelBundle::from_min_max(
        Vec2::new(-1000., -400.),
        Vec2::new(1000., -200.),
    ));
    // Left shelter roof
    commands.spawn(LevelBundle::from_min_max(
        Vec2::new(-700., -100.),
        Vec2::new(-400., 150.),
    ));
    // Left shelter wall
    commands.spawn(LevelBundle::from_min_max(
        Vec2::new(-700., -200.),
        Vec2::new(-600., -100.),
    ));
    // Middle shelter
    commands.spawn(LevelBundle::from_min_max(
        Vec2::new(-100., -60.),
        Vec2::new(100., -40.),
    ));
    // Right shelter roof
    commands.spawn(LevelBundle::from_min_max(
        Vec2::new(400., -100.),
        Vec2::new(700., 150.),
    ));
    // Right shelter wall
    commands.spawn(LevelBundle::from_min_max(
        Vec2::new(600., -200.),
        Vec2::new(700., -100.),
    ));

    spawn_player(commands, Vec3::new(-550., -184., 1.));
}

fn despawn_level(mut commands: Commands, query: Query<Entity, With<Level>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
