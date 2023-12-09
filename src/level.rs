use bevy::prelude::*;

use crate::{app_state::AppState, collider::Collider};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_level);
    }
}

#[derive(Component)]
pub struct Level;

#[derive(Bundle)]
struct LevelBundle {
    sprite: SpriteBundle,
    collider: Collider,
    level: Level,
}

impl LevelBundle {
    fn from_center_size(position: Vec2, size: Vec2) -> Self {
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(size),
                    ..default()
                },
                transform: Transform::from_translation(position.extend(0.)),
                ..default()
            },
            collider: Collider::from_size(size),
            level: Level,
        }
    }
}

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
}
