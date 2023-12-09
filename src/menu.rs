use crate::{app_state::*, menu_button::*};
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), setup_menu)
            .add_systems(Update, click_button.run_if(in_state(AppState::Menu)))
            .add_systems(OnExit(AppState::Menu), cleanup_menu);
    }
}

#[derive(Component)]
struct Menu;

fn setup_menu(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            children
                .spawn((
                    MenuButtonBundle::default().with_width(Val::Px(140.0)),
                    MenuAction::ChangeState(AppState::InGame),
                ))
                .with_children(|parent| {
                    parent.spawn(MenuButtonLabelBundle::from_text("Play"));
                });
        });
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceAround,
                    bottom: Val::Px(5.),
                    width: Val::Percent(100.),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            children
                .spawn((
                    MenuButtonBundle::transparent(),
                    MenuAction::OpenLink("https://bevyengine.org"),
                ))
                .with_children(|parent| {
                    parent.spawn(MenuButtonLabelBundle::from_text("Made with Bevy").small());
                });
            children
                .spawn((
                    MenuButtonBundle::transparent(),
                    MenuAction::OpenLink("https://github.com/NiklasEi/bevy_game_template"),
                ))
                .with_children(|parent| {
                    parent.spawn(
                        MenuButtonLabelBundle::from_text("Made with bevy_game_template").small(),
                    );
                });
        });
}

#[derive(Component)]
enum MenuAction {
    ChangeState(AppState),
    OpenLink(&'static str),
}

fn click_button(
    mut next_state: ResMut<NextState<AppState>>,
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
            MenuAction::OpenLink(link) => {
                if let Err(error) = webbrowser::open(link) {
                    warn!("Failed to open link {error:?}");
                }
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
