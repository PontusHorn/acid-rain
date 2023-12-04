use crate::actions::Actions;
use crate::level::Level;
use crate::velocity::{update_position, Velocity};
use crate::GameState;
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    jump_state: JumpState,
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
                update_velocity
                    .before(update_position)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::splat(32.)),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., -150., 1.)),
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
) {
    let player_movement = actions.player_movement.unwrap_or(Vec2::ZERO);
    let delta = time.delta_seconds();

    for (mut player_velocity, mut player_transform, mut player) in &mut player_query {
        let new_velocity_x = get_velocity_x(player_velocity.0.x, player_movement.x, delta);
        let (mut new_velocity_y, mut new_jump_state) = get_velocity_y(
            player_velocity.0.y,
            player_movement.y,
            &player.jump_state,
            delta,
        );

        let distance_above_ground =
            player_transform.translation.y + new_velocity_y * delta - Level::GROUND_Y;
        if new_jump_state == JumpState::Falling && distance_above_ground <= 0. {
            new_velocity_y = 0.;
            new_jump_state = JumpState::Grounded;
            player_transform.translation.y = Level::GROUND_Y;
        }
        player_velocity.0.x = new_velocity_x;
        player_velocity.0.y = new_velocity_y;
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
                (0., JumpState::Grounded)
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
