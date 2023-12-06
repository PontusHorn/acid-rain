use bevy::prelude::*;

use crate::GameState;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_level);
    }
}

#[derive(Component)]
pub struct Level(pub Vec2);

impl Level {
    pub fn size(&self) -> Vec2 {
        self.0
    }

    pub fn rect(&self, transform: &Transform) -> Rect {
        Rect::from_center_size(transform.translation.truncate(), self.size())
    }
}

#[derive(Bundle)]
struct LevelBundle {
    sprite: SpriteBundle,
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
            level: Level(size),
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
