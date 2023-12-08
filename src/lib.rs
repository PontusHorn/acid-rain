#![allow(clippy::type_complexity)]

mod actions;
mod collider;
mod color;
mod health;
mod level;
mod loading;
mod menu;
mod player;
mod power;
mod rain;
mod shield;
mod velocity;

use crate::actions::ActionsPlugin;
use crate::health::HealthPlugin;
use crate::level::LevelPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;
use crate::power::PowerPlugin;
use crate::rain::RainPlugin;
use crate::shield::ShieldPlugin;
use crate::velocity::VelocityPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{
    EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin,
};
use bevy::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugins((
            LoadingPlugin,
            MenuPlugin,
            LevelPlugin,
            ActionsPlugin,
            PlayerPlugin,
            ShieldPlugin,
            HealthPlugin,
            PowerPlugin,
            RainPlugin,
            VelocityPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((
                FrameTimeDiagnosticsPlugin,
                EntityCountDiagnosticsPlugin,
                LogDiagnosticsPlugin::default(),
            ));
        }
    }
}
