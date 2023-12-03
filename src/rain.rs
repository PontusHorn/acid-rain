use crate::{velocity::Velocity, GameState};
use bevy::{prelude::*, sprite::Anchor};
use rand::prelude::*;

pub struct RainPlugin;

#[derive(Component)]
pub struct Rain;

impl Plugin for RainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_rain, despawn_rain).run_if(in_state(GameState::Playing)),
        );
    }
}

const DENSITY: i8 = 3;
const HEIGHT: f32 = 16.;
const VELOCITY: Vec2 = Vec2::new(50., -800.);

fn spawn_rain(mut commands: Commands, camera_query: Query<&OrthographicProjection>) {
    let mut rng = thread_rng();
    let camera_projection = camera_query.single();

    for _ in 0..DENSITY {
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(1., HEIGHT)),
                    anchor: Anchor::BottomCenter,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    rng.gen_range(camera_projection.area.min.x..camera_projection.area.max.x),
                    camera_projection.area.max.y,
                    2.,
                ))
                .with_rotation(Quat::from_rotation_z(0.2)),
                ..default()
            })
            .insert(Velocity(VELOCITY))
            .insert(Rain);
    }
}

fn despawn_rain(
    mut commands: Commands,
    rain_query: Query<(Entity, &Transform), With<Rain>>,
    camera_query: Query<&OrthographicProjection>,
) {
    let camera_projection = camera_query.single();
    for (entity, rain_transform) in rain_query.iter() {
        if rain_transform.translation.y < camera_projection.area.min.y - HEIGHT {
            commands.entity(entity).despawn();
        }
    }
}
