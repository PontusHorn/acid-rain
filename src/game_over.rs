use bevy::prelude::*;

use crate::{app_state::*, ui::*};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), spawn_game_over_screen)
            .add_systems(OnExit(GameState::GameOver), despawn_game_over_screen)
            .add_systems(Update, start_over.run_if(in_state(GameState::GameOver)));
    }
}

#[derive(Component)]
pub struct GameOverScreen;

fn spawn_game_over_screen(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::rgba(0., 0., 0., 0.).into(),
                ..default()
            },
            FadeIn::from_seconds(0.5).with_background_alpha(0.8),
            GameOverScreen,
        ))
        .with_children(|children| {
            children
                .spawn((
                    MenuButtonBundle::default().with_width(Val::Px(200.0)),
                    MenuAction::ChangeState(GameState::Playing),
                    FadeIn::from_seconds(1.),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        MenuButtonLabelBundle::from_text("Try again").with_alpha(0.),
                        FadeIn::from_seconds(1.).with_background_alpha(0.),
                    ));
                });
        });
}

fn despawn_game_over_screen(mut commands: Commands, query: Query<Entity, With<GameOverScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component)]
enum MenuAction {
    ChangeState(GameState),
}

fn start_over(
    mut next_state: ResMut<NextState<GameState>>,
    interaction_query: Query<(&Interaction, &MenuAction), (Changed<Interaction>, With<MenuButton>)>,
) {
    for (interaction, action) in interaction_query.iter() {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match action {
            MenuAction::ChangeState(state) => {
                next_state.set(state.clone());
            }
        }
    }
}
