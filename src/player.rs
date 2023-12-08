use crate::actions::Actions;
use crate::collider::Collider;
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
    const SIZE: Vec2 = Vec2::splat(32.);
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
                    fade_out_damage.before(get_hit_by_rain),
                    get_hit_by_rain.after(splash_rain),
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

const BASE_COLOR: Color = Color::rgb(0., 0.5, 0.8);

fn spawn_player(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: BASE_COLOR,
                custom_size: Some(Player::SIZE),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-400., -180., 1.)),
            ..default()
        })
        .insert(Velocity(Vec2::ZERO))
        .insert(Collider::from_center_size(
            Vec2::new(0., Player::SIZE.y / 2.),
            Player::SIZE,
        ))
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
    mut player_query: Query<(&mut Velocity, &mut Transform, &Collider, &mut Player)>,
    level_query: Query<(&Transform, &Collider), (With<Level>, Without<Player>)>,
) {
    let player_movement = actions.player_movement.unwrap_or(Vec2::ZERO);
    let delta = time.delta_seconds();

    for (mut player_velocity, mut player_transform, player_collider, mut player) in
        player_query.iter_mut()
    {
        let new_velocity_x = get_velocity_x(player_velocity.0.x, player_movement.x, delta);
        let (new_velocity_y, mut new_jump_state) = get_velocity_y(
            player_velocity.0.y,
            player_movement.y,
            &player.jump_state,
            delta,
        );
        let mut new_velocity = Vec2::new(new_velocity_x, new_velocity_y);
        let mut new_transform = player_transform
            .with_translation(player_transform.translation + new_velocity.extend(0.) * delta);
        let mut is_on_ground = false;

        for (level_transform, level_collider) in level_query.iter() {
            let player_rect = player_collider.rect(&new_transform);
            let level_rect = level_collider.rect(level_transform);
            let collision = collide(
                new_transform.translation,
                player_rect.size(),
                level_transform.translation,
                level_rect.size(),
            );
            match collision {
                None => continue,
                Some(Collision::Top) | Some(Collision::Inside) => {
                    new_velocity.y = 0.;
                    new_jump_state = JumpState::Grounded;
                    is_on_ground = true;
                    new_transform.translation.y = level_rect.max.y;
                }
                Some(Collision::Bottom) => {
                    new_velocity.y = 0.;
                    new_transform.translation.y = level_rect.min.y - Player::SIZE.y;
                }
                Some(Collision::Left) => {
                    new_velocity.x = 0.;
                    new_transform.translation.x = level_rect.min.x - Player::SIZE.x / 2.;
                }
                Some(Collision::Right) => {
                    new_velocity.x = 0.;
                    new_transform.translation.x = level_rect.max.x + Player::SIZE.x / 2.;
                }
            }
        }

        if !is_on_ground && player.jump_state == JumpState::Grounded {
            new_jump_state = JumpState::Falling;
        }

        player_transform.apply(&new_transform);
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

fn get_hit_by_rain(
    mut rain_player_hit: EventReader<RainPlayerHit>,
    mut player_query: Query<&mut Sprite, With<Player>>,
    mut health: ResMut<Health>,
) {
    let mut player_sprite = player_query.single_mut();
    for _ in rain_player_hit.read() {
        if health.0 > 0 {
            health.0 -= 1;
            player_sprite.color = Color::rgb(0.5, 0.19, 0.38);
        }
    }
}

fn fade_out_damage(time: Res<Time>, mut player_query: Query<&mut Sprite, With<Player>>) {
    let delta = time.delta_seconds();
    for mut player_sprite in player_query.iter_mut() {
        if player_sprite.color != BASE_COLOR {
            if colors_equal(player_sprite.color, BASE_COLOR) {
                player_sprite.color = BASE_COLOR;
            } else {
                player_sprite.color =
                    lerp_colors(player_sprite.color, BASE_COLOR, (3. * delta).min(1.));
            }
        }
    }
}

fn colors_equal(lhs: Color, rhs: Color) -> bool {
    let max_diff = 1. / 255.;
    (lhs.r() - rhs.r()).abs() < max_diff
        && (lhs.g() - rhs.g()).abs() < max_diff
        && (lhs.b() - rhs.b()).abs() < max_diff
}

fn lerp_colors(lhs: Color, rhs: Color, t: f32) -> Color {
    lhs.as_rgba_linear() * (1.0 - t) + rhs.as_rgba_linear() * t
}
