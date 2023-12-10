use bevy::prelude::*;

use crate::collider::Collider;

#[derive(Component)]
pub struct Level;

#[derive(Bundle)]
pub struct LevelBundle {
    sprite: SpriteBundle,
    collider: Collider,
    level: Level,
}

impl LevelBundle {
    pub fn from_center_size(position: Vec2, size: Vec2) -> Self {
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
