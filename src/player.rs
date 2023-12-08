use crate::actions::Actions;
use crate::health::Health;
use crate::level::Level;
use crate::rain::{splash_rain, RainPlayerHit};
use crate::velocity::{update_position, Velocity};
use crate::GameState;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use bevy::sprite::Anchor;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    jump_state: JumpState,
}

impl Player {
    pub const SIZE: Vec2 = Vec2::splat(32.);

    pub fn center(&self, translation: &Vec3) -> Vec3 {
        Vec3::new(0., Self::SIZE.y / 2., 0.) + *translation
    }
}

#[derive(PartialEq, Debug)]
pub enum JumpState {
    Grounded,
    Jumping(f32),
    Falling,
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                Update,
                (
                    update_velocity.before(update_position),
                    get_hit_by_rain.after(splash_rain),
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)]
pub struct CoyoteTimer(pub Timer);

fn spawn_player(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0., 100., 150.),
                custom_size: Some(Player::SIZE),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-400., -180., 1.)),
            ..default()
        })
        .insert(Velocity(Vec2::ZERO))
        .insert(Player {
            jump_state: JumpState::Falling,
        });
}

const X_SPEED: f32 = 200.;
const ACCELERATION_X: f32 = 10.;
const DECELERATION_X: f32 = 15.;

fn update_velocity(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<(&mut Velocity, &mut Transform, &mut Player)>,
    level_query: Query<(&Transform, &Level), Without<Player>>,
) {
    let player_movement = actions.player_movement.unwrap_or(Vec2::ZERO);
    let delta = time.delta_seconds();

    for (mut player_velocity, mut player_transform, mut player) in player_query.iter_mut() {
        let new_velocity_x = get_velocity_x(player_velocity.0.x, player_movement.x, delta);
        let (new_velocity_y, mut new_jump_state) = get_velocity_y(
            player_velocity.0.y,
            player_movement.y,
            &player.jump_state,
            delta,
        );
        let mut new_velocity = Vec2::new(new_velocity_x, new_velocity_y);
        let mut is_on_ground = false;

        for (level_transform, level) in level_query.iter() {
            let new_translation = player_transform.translation + new_velocity.extend(0.) * delta;
            let collision = collide(
                player.center(&new_translation),
                Player::SIZE,
                level_transform.translation,
                level.size(),
            );
            let level_rect = level.rect(level_transform);
            match collision {
                None => continue,
                Some(Collision::Top) => {
                    new_velocity.y = 0.;
                    new_jump_state = JumpState::Grounded;
                    is_on_ground = true;
                    player_transform.translation.y = level_rect.max.y;
                }
                Some(Collision::Bottom) => {
                    new_velocity.y = 0.;
                    player_transform.translation.y = level_rect.min.y - Player::SIZE.y;
                }
                Some(Collision::Left) => {
                    new_velocity.x = 0.;
                    player_transform.translation.x = level_rect.min.x - Player::SIZE.x / 2.;
                }
                Some(Collision::Right) => {
                    new_velocity.x = 0.;
                    player_transform.translation.x = level_rect.max.x + Player::SIZE.x / 2.;
                }
                Some(Collision::Inside) => {
                    new_velocity = Vec2::ZERO;
                }
            }
        }

        if !is_on_ground && player.jump_state == JumpState::Grounded {
            new_jump_state = JumpState::Falling;
        }

        player_velocity.0 = new_velocity;
        player.jump_state = new_jump_state;
    }
}

fn get_velocity_x(velocity_x: f32, movement_x: f32, delta: f32) -> f32 {
    let acceleration_x = if movement_x == 0. {
        DECELERATION_X
    } else {
        ACCELERATION_X
    } * delta;
    let target_vel_x = movement_x * X_SPEED;
    let mut new_vel_x = velocity_x;
    new_vel_x += (target_vel_x - velocity_x) * acceleration_x;
    if new_vel_x.abs() < 0. {
        new_vel_x = 0.0;
    }
    new_vel_x
}

const JUMP_SPEED: f32 = 800.;
const JUMP_GRAVITY: f32 = -1000.;
const FALL_SPEED: f32 = 400.;
const FALL_GRAVITY: f32 = -1000.;

fn get_velocity_y(
    velocity_y: f32,
    movement_y: f32,
    jump_state: &JumpState,
    delta: f32,
) -> (f32, JumpState) {
    match jump_state {
        JumpState::Grounded => {
            if movement_y > 0. {
                (JUMP_SPEED, JumpState::Jumping(1.))
            } else {
                (-1., JumpState::Grounded)
            }
        }
        JumpState::Jumping(jump_power) => {
            if movement_y < 0. {
                (0., JumpState::Falling)
            } else {
                let jump_power_loss = (0.9 - movement_y * 0.85) * delta;
                let new_jump_power = jump_power * (1. - jump_power_loss);
                let new_velocity_y = velocity_y * new_jump_power + JUMP_GRAVITY * delta;
                (
                    new_velocity_y,
                    if new_velocity_y >= 0. {
                        JumpState::Jumping(new_jump_power)
                    } else {
                        JumpState::Falling
                    },
                )
            }
        }
        JumpState::Falling => {
            let new_velocity_y = (velocity_y + FALL_GRAVITY * delta).max(-FALL_SPEED);
            (new_velocity_y, JumpState::Falling)
        }
    }
}

fn get_hit_by_rain(mut rain_player_hit: EventReader<RainPlayerHit>, mut health: ResMut<Health>) {
    for _ in rain_player_hit.read() {
        if health.0 > 0 {
            health.0 -= 1;
        }
    }
}
