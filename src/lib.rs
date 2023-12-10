#![allow(clippy::type_complexity)]

mod actions;
mod app_state;
mod collider;
mod color;
mod game_over;
mod health;
mod level;
mod levels;
mod loading;
mod menu;
mod player;
mod power;
mod rain;
mod shield;
mod ui;
mod velocity;

use crate::actions::ActionsPlugin;
use crate::app_state::AppStatePlugin;
use crate::game_over::GameOverPlugin;
use crate::health::HealthPlugin;
use crate::levels::LevelsPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;
use crate::power::PowerPlugin;
use crate::rain::RainPlugin;
use crate::shield::ShieldPlugin;
use crate::ui::UiPlugin;
use crate::velocity::VelocityPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{
    EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin,
};
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AppStatePlugin,
            LoadingPlugin,
            UiPlugin,
            MenuPlugin,
            LevelsPlugin,
            ActionsPlugin,
            GameOverPlugin,
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
