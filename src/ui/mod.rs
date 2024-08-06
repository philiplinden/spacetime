mod camera;
mod cursor;
mod shell;

use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct UserInterfacePlugins;

impl PluginGroup for UserInterfacePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(bevy_egui::EguiPlugin)
            .add(camera::plugin)
            .add(shell::plugin)
            .add(DebugExtrasPlugin)
    }
}

/// Optionally adds more debug utility interfaces to the UI
struct DebugExtrasPlugin;

impl Plugin for DebugExtrasPlugin {
    fn build(&self, app: &mut App) {

        #[cfg(feature = "dev")]
        app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::default());
    }
}
