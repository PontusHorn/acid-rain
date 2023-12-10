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
    commands.spawn(LevelBundle::from_center_size(
        Vec2::new(0., -300.),
        Vec2::new(2000., 200.),
    ));
    commands.spawn(LevelBundle::from_center_size(
        Vec2::new(-400., 0.),
        Vec2::new(200., 100.),
    ));
    commands.spawn(LevelBundle::from_center_size(
        Vec2::new(100., -50.),
        Vec2::new(150., 20.),
    ));

    spawn_player(commands, Vec3::new(-400., -180., 1.));
}

fn despawn_level(mut commands: Commands, query: Query<Entity, With<Level>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
