use crate::app_state::*;
use bevy::prelude::*;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Health(100))
            .add_systems(OnEnter(AppState::InGame), spawn_health_display)
            .add_systems(OnExit(AppState::InGame), despawn_health_display)
            .add_systems(OnExit(GameState::Playing), reset_health)
            .add_systems(
                Update,
                update_health_display.run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Resource)]
pub struct Health(pub u8);

#[derive(Component)]
struct HealthDisplay;

#[derive(Component)]
struct HealthBar;

fn spawn_health_display(mut commands: Commands, health: Res<Health>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    top: Val::Px(10.0),
                    right: Val::Px(10.0),
                    width: Val::Px(100.0),
                    height: Val::Px(25.0),
                    padding: UiRect::all(Val::Px(3.)),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            },
            HealthDisplay,
        ))
        .with_children(|children| {
            children.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(health.0 as f32),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: Color::GREEN.into(),
                    ..default()
                },
                HealthBar,
            ));
        });
}

fn despawn_health_display(mut commands: Commands, query: Query<Entity, With<HealthDisplay>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn reset_health(mut health: ResMut<Health>) {
    health.0 = 100;
}

fn update_health_display(
    mut health_bar_query: Query<&mut Style, With<HealthBar>>,
    health: Res<Health>,
) {
    for mut style in health_bar_query.iter_mut() {
        style.width = Val::Percent(health.0 as f32);
    }
}
