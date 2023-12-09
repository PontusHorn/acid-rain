use bevy::prelude::*;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    #[default]
    Loading,
    Menu,
    InGame,
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    None,
    Playing,
    GameOver,
}

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_state::<GameState>()
            .add_systems(OnEnter(AppState::InGame), set_game_state_playing)
            .add_systems(OnExit(AppState::InGame), set_game_state_none);
    }
}

fn set_game_state_playing(mut playing_state: ResMut<NextState<GameState>>) {
    playing_state.set(GameState::Playing);
}

fn set_game_state_none(mut playing_state: ResMut<NextState<GameState>>) {
    playing_state.set(GameState::None);
}
