use std::f32::consts::{FRAC_PI_2, PI};

use crate::{
    collider::Collider,
    player::Player,
    velocity::{update_position, Velocity},
    GameState,
};
use bevy::{
    prelude::*,
    sprite::{collide_aabb::*, Anchor},
};
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
const SIZE: Vec2 = Vec2::new(8., 12.);

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
    target_query: Query<(&Transform, &Collider, Option<&Player>), Without<Rain>>,
    mut rain_player_hit_writer: EventWriter<RainPlayerHit>,
) {
    let rng = &mut thread_rng();

    for (mut rain, mut rain_velocity, mut rain_transform) in rain_query.iter_mut() {
        let rain_translation = rain_transform.translation;
        match rain.0 {
            RainState::Falling => {
                for (target_transform, target_collider, player) in target_query.iter() {
                    let Some(target_collision) = collide(
                        rain_translation,
                        SIZE,
                        target_transform.translation,
                        target_collider.size,
                    ) else {
                        continue;
                    };

                    let target_rect = target_collider.rect(target_transform);
                    handle_collision(
                        target_collision,
                        rng,
                        &target_rect,
                        (&mut rain, &mut rain_velocity, &mut rain_transform),
                    );

                    if player.is_some() {
                        rain_player_hit_writer.send(RainPlayerHit);
                    }
                    break;
                }
            }
            RainState::Splashing => {
                rain_transform.scale.y *= 0.7;
            }
        }
    }
}

fn handle_collision(
    collision: Collision,
    rng: &mut ThreadRng,
    rect: &Rect,
    (rain, rain_velocity, rain_transform): (&mut Rain, &mut Velocity, &mut Transform),
) {
    match collision {
        Collision::Top | Collision::Inside => {
            splash_against_top_side(rng, rect, (rain, rain_velocity, rain_transform));
        }
        Collision::Left => {
            run_along_left_side(rng, rect, (rain_velocity, rain_transform));
        }
        _ => {
            // Other collisions are very unlikely to happen due
            // to the direction of the rain fall, so just scale
            // it down to let despawn_rain() handle it.
            rain.0 = RainState::Splashing;
            rain_transform.scale.x = 0.;
        }
    }
}

fn splash_against_top_side(
    rng: &mut ThreadRng,
    rect: &Rect,
    (rain, rain_velocity, rain_transform): (&mut Rain, &mut Velocity, &mut Transform),
) {
    rain.0 = RainState::Splashing;
    rain_transform.translation.y = rect.max.y;
    rain_transform.scale.x *= rng.gen_range(0.2..0.6);
    let splash_angle_offset = rng.gen_range(-FRAC_PI_2..FRAC_PI_2);
    let splash_angle = FRAC_PI_2 + splash_angle_offset;
    let splash_speed = SPEED * rng.gen_range(0.1..0.4) * (0.3 + splash_angle_offset.abs());
    rain_transform.rotate_local_z(splash_angle - ANGLE);
    rain_velocity.0 = Vec2::from_angle(splash_angle) * splash_speed;
}

fn run_along_left_side(
    rng: &mut ThreadRng,
    rect: &Rect,
    (rain_velocity, rain_transform): (&mut Velocity, &mut Transform),
) {
    rain_transform.translation.x = rect.min.x;
    rain_transform.scale.x *= rng.gen_range(0.7..0.9);
    let splash_angle = PI * 1.5 - rng.gen_range(0.0..0.03);
    let splash_speed = SPEED * rng.gen_range(0.4..0.8);
    rain_transform.rotation = Quat::from_rotation_z(splash_angle);
    rain_velocity.0 = Vec2::from_angle(splash_angle) * splash_speed;
}

fn despawn_rain(
    mut commands: Commands,
    rain_query: Query<(Entity, &Transform), With<Rain>>,
    camera_query: Query<&OrthographicProjection>,
) {
    let camera_projection = camera_query.single();
    for (entity, rain_transform) in rain_query.iter() {
        if rain_transform.scale.y < 0.1 {
            commands.entity(entity).despawn();
        }

        if rain_transform.translation.y < camera_projection.area.min.y - 100. {
            commands.entity(entity).despawn();
        }
    }
}
