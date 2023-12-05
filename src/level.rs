use bevy::prelude::*;

use crate::{collider::Collider, GameState};

pub struct LevelPlugin;

#[derive(Component)]
pub struct Level;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_level);
    }
}

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
            collider: Collider::new(Rect::from_center_size(
                Vec2::ZERO,
                Vec2::new(size.x, size.y),
            )),
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
}
