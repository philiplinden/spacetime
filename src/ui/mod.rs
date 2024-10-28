use bevy::prelude::*;
use bevy_egui::EguiPlugin;

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiState>()
            .add_plugins((EguiPlugin))
    }
}

#[derive(Resource, Reflect)]
pub struct UiState {
    pub visible: bool,
    pub show_debug: bool,
    pub show_keys: bool,
}

impl Default for UiState {
    fn default() -> Self {
        UiState {
            visible: true,
            show_debug: false,
            show_keys: false,
        }
    }
}
