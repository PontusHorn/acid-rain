use std::f32::consts::PI;

use crate::{
    collider::Collider,
    level::Level,
    player::Player,
    velocity::{update_position, Velocity},
    GameState,
};
use bevy::{prelude::*, sprite::Anchor};
use rand::prelude::*;

pub struct RainPlugin;

#[derive(Component)]
pub struct Rain(RainState);

enum RainState {
    Falling,
    Splashing,
}

impl Plugin for RainPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RainPlayerHit>().add_systems(
            Update,
            (
                spawn_rain,
                (splash_rain, despawn_rain).after(update_position),
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

const DENSITY: i8 = 16;
const ANGLE: f32 = -1.4;
const SPEED: f32 = 800.;

fn spawn_rain(mut commands: Commands, camera_query: Query<&OrthographicProjection>) {
    let mut rng = thread_rng();
    let camera_projection = camera_query.single();

    for _ in 0..DENSITY {
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(rng.gen_range(4.0..=16.0), 1.)),
                    ..default()
                },
                transform: Transform::from_rotation(Quat::from_rotation_z(ANGLE)).with_translation(
                    Vec3::new(
                        rng.gen_range(
                            (camera_projection.area.min.x - 200.)..camera_projection.area.max.x,
                        ),
                        camera_projection.area.max.y,
                        2.,
                    ),
                ),
                ..default()
            })
            .insert(Anchor::CenterRight)
            .insert(Velocity(Vec2::from_angle(ANGLE) * SPEED))
            .insert(Rain(RainState::Falling));
    }
}

#[derive(Event)]
pub struct RainPlayerHit;

pub fn splash_rain(
    mut rain_query: Query<(&mut Rain, &mut Velocity, &mut Transform)>,
    player_query: Query<(&Transform, &Collider), (With<Player>, Without<Rain>)>,
    level_query: Query<(&Transform, &Collider), (With<Level>, Without<Rain>)>,
    mut rain_player_hit_writer: EventWriter<RainPlayerHit>,
) {
    let rng = &mut thread_rng();
    let (player_transform, player_collider) = player_query.single();
    let player_collider = player_collider.translate(player_transform.translation.truncate());

    for (mut rain, mut rain_velocity, mut rain_transform) in rain_query.iter_mut() {
        let rain_position = rain_transform.translation.truncate();
        match rain.0 {
            RainState::Falling => {
                for (level_transform, level_collider) in level_query.iter() {
                    let level_collider =
                        level_collider.translate(level_transform.translation.truncate());
                    if !level_collider.contains(rain_position) {
                        continue;
                    }
                    rain.0 = RainState::Splashing;
                    rain_transform.translation.y = level_collider.0.max.y;
                    rain_transform.scale.x *= rng.gen_range(0.4..0.8);
                    let splash_angle = PI / 2. + rng.gen_range(-0.6..0.6);
                    let splash_speed = SPEED * rng.gen_range(0.2..0.8);
                    rain_transform.rotate_local_z(splash_angle - ANGLE);
                    rain_velocity.0 = Vec2::from_angle(splash_angle) * splash_speed;
                    break;
                }

                if player_collider.contains(rain_position) {
                    rain_player_hit_writer.send(RainPlayerHit);
                }
            }
            RainState::Splashing => {
                rain_transform.scale.y *= 0.7;
            }
        }
    }
}

fn despawn_rain(mut commands: Commands, rain_query: Query<(Entity, &Transform), With<Rain>>) {
    for (entity, rain_transform) in rain_query.iter() {
        if rain_transform.scale.y < 0.1 {
            commands.entity(entity).despawn();
        }
    }
}
