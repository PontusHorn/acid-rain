mod fade_in;
mod menu_button;

use bevy::prelude::*;
pub use fade_in::*;
pub use menu_button::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((menu_button::MenuButtonPlugin, fade_in::FadeInPlugin));
    }
}
