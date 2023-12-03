use crate::GameState;
use bevy::prelude::*;

pub struct VelocityPlugin;

#[derive(Component)]
pub struct Velocity(pub Vec2);

impl Plugin for VelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_position).run_if(in_state(GameState::Playing)),
        );
    }
}

fn update_position(time: Res<Time>, mut velocity_query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut velocity_query {
        transform.translation += velocity.0.extend(0.) * time.delta_seconds();
    }
}
