pub mod labels;
pub mod performance;

use performance::FpsMonitorPlugin;
// use labels::EntityLabelPlugin;

use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct DebugUiPlugin;

impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            FpsMonitorPlugin,
            // EntityLabelPlugin,
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        ));
    }
}
