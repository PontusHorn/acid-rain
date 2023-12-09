#![allow(clippy::type_complexity)]

mod actions;
mod app_state;
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
use crate::app_state::AppStatePlugin;
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

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AppStatePlugin,
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
