use std::f32::consts::PI;

use crate::{
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

const DENSITY: i8 = 3;
const LENGTH: f32 = 16.;
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
                    custom_size: Some(Vec2::new(LENGTH, 1.)),
                    ..default()
                },
                transform: Transform::from_rotation(Quat::from_rotation_z(ANGLE)).with_translation(
                    Vec3::new(
                        rng.gen_range(camera_projection.area.min.x..camera_projection.area.max.x),
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
    player_query: Query<&Transform, (With<Player>, Without<Rain>)>,
    mut rain_player_hit_writer: EventWriter<RainPlayerHit>,
) {
    let rng = &mut thread_rng();
    let player_transform = player_query.single();

    for (mut rain, mut rain_velocity, mut rain_transform) in rain_query.iter_mut() {
        match rain.0 {
            RainState::Falling => {
                if rain_transform.translation.y <= Level::GROUND_Y {
                    rain.0 = RainState::Splashing;
                    rain_transform.translation.y = Level::GROUND_Y;
                    rain_transform.scale.x *= rng.gen_range(0.4..0.8);
                    let splash_angle = PI / 2. + rng.gen_range(-0.6..0.6);
                    let splash_speed = SPEED * rng.gen_range(0.2..0.8);
                    rain_transform.rotate_local_z(splash_angle - ANGLE);
                    rain_velocity.0 = Vec2::from_angle(splash_angle) * splash_speed;
                } else if is_rain_on_player(&rain_transform, player_transform) {
                    rain_player_hit_writer.send(RainPlayerHit);
                }
            }
            RainState::Splashing => {
                rain_transform.scale.y *= 0.7;
            }
        }
    }
}

fn is_rain_on_player(rain_transform: &Transform, player_transform: &Transform) -> bool {
    let player_rect = Rect::from_center_size(
        Vec2::new(
            player_transform.translation.x,
            player_transform.translation.y + Player::SIZE.y / 2.,
        ),
        Player::SIZE,
    );
    player_rect.contains(rain_transform.translation.truncate())
}

fn despawn_rain(mut commands: Commands, rain_query: Query<(Entity, &Transform), With<Rain>>) {
    for (entity, rain_transform) in rain_query.iter() {
        if rain_transform.scale.y < 0.1 {
            commands.entity(entity).despawn();
        }
    }
}
