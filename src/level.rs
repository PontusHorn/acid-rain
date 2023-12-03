use bevy::{prelude::*, sprite::Anchor};

use crate::GameState;

pub struct LevelPlugin;

#[derive(Component)]
pub struct Level;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_level);
    }
}

fn spawn_level(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(2000., 500.)),
                anchor: Anchor::TopCenter,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., -200., 0.)),
            ..default()
        })
        .insert(Level);
}
