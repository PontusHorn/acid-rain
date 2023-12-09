use crate::app_state::*;
use bevy::prelude::*;

pub struct PowerPlugin;

impl Plugin for PowerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Power(1.))
            .add_systems(OnEnter(AppState::InGame), spawn_power_display)
            .add_systems(
                Update,
                update_power_display.run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Resource)]
pub struct Power(pub f32);

#[derive(Component)]
struct PowerBar;

fn spawn_power_display(mut commands: Commands, power: Res<Power>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                top: Val::Px(45.0),
                right: Val::Px(10.0),
                width: Val::Px(100.0),
                height: Val::Px(25.0),
                padding: UiRect::all(Val::Px(3.)),
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        })
        .with_children(|children| {
            children.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(power.0 * 100.),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: Color::CYAN.into(),
                    ..default()
                },
                PowerBar,
            ));
        });
}

fn update_power_display(mut power_bar_query: Query<&mut Style, With<PowerBar>>, power: Res<Power>) {
    for mut style in power_bar_query.iter_mut() {
        style.width = Val::Percent(power.0 * 100.);
    }
}
