use bevy::prelude::*;

use super::window::window;

pub struct GamePlugin;

impl Plugin for GamePlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins(window::WindowSettingsPlugin);
    }
}
