use crate::app_state::GameState;
use crate::collider::Collider;
use crate::color::*;
use crate::power::Power;
use crate::rain::*;
use bevy::prelude::*;

pub struct ShieldPlugin;

impl Plugin for ShieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                activate_shield,
                fade_out_damage.before(get_hit_by_rain),
                get_hit_by_rain.after(splash_rain),
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Component)]
pub struct Shield;

impl Shield {
    const COLOR_BASE: Color = Color::rgba(0.0, 1.0, 1.0, 0.5);
    const COLOR_HIT: Color = Color::rgba(1.0, 1.0, 1.0, 0.5);
    const SIZE: Vec2 = Vec2::splat(64.);
}

#[derive(Bundle)]
pub struct ShieldBundle {
    sprite: SpriteBundle,
    collider: Collider,
    rain_hit_listener: RainHitListener,
    shield: Shield,
}

impl ShieldBundle {
    pub fn new(transform: Transform) -> Self {
        Self {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Shield::COLOR_BASE,
                    custom_size: Some(Shield::SIZE),
                    ..default()
                },
                transform,
                visibility: Visibility::Hidden,
                ..default()
            },
            collider: Collider::from_size(Shield::SIZE).with_solid(false),
            rain_hit_listener: RainHitListener,
            shield: Shield,
        }
    }
}

fn activate_shield(
    mut shield_query: Query<(&mut Collider, &mut Visibility), With<Shield>>,
    mut power: ResMut<Power>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let shield_active = keyboard_input.pressed(KeyCode::Space);
    let delta = time.delta_seconds() * 60.;
    let power_cost = 0.03 * delta;
    let power_recharge = 0.02 * delta;

    for (mut collider, mut visibility) in shield_query.iter_mut() {
        if shield_active && power.0 >= power_cost {
            power.0 -= power_cost;
            if !collider.solid || *visibility == Visibility::Hidden {
                collider.solid = true;
                *visibility = Visibility::Visible;
            }
        } else if collider.solid || *visibility == Visibility::Visible {
            collider.solid = false;
            *visibility = Visibility::Hidden;
        }
    }

    if !shield_active && power.0 < 1. {
        power.0 = (power.0 + power_recharge).min(1.);
    }
}

fn get_hit_by_rain(
    mut rain_hit: EventReader<RainHit>,
    mut shield_query: Query<(&mut Sprite, Entity), With<Shield>>,
) {
    for (mut shield_sprite, shield_entity) in shield_query.iter_mut() {
        for RainHit(entity) in rain_hit.read() {
            if shield_entity == *entity {
                shield_sprite.color = Shield::COLOR_HIT;
            }
        }
    }
}

fn fade_out_damage(time: Res<Time>, mut shield_query: Query<&mut Sprite, With<Shield>>) {
    let delta = time.delta_seconds();
    for mut shield_sprite in shield_query.iter_mut() {
        if shield_sprite.color != Shield::COLOR_BASE {
            if colors_equal(shield_sprite.color, Shield::COLOR_BASE) {
                shield_sprite.color = Shield::COLOR_BASE;
            } else {
                shield_sprite.color = lerp_colors(
                    shield_sprite.color,
                    Shield::COLOR_BASE,
                    (3. * delta).min(1.),
                );
            }
        }
    }
}
