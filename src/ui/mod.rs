mod camera;
mod cursor;
mod diagnostics;
mod shell;

use bevy::{app::PluginGroupBuilder, prelude::*};

// We map this to the function format for consistency
pub(super) fn plugin(app: &mut App) {
    app.add_plugins(UserInterfacePlugins);
}

pub struct UserInterfacePlugins;

impl PluginGroup for UserInterfacePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(bevy_egui::EguiPlugin)
            .add(camera::plugin)
            .add(cursor::plugin)
            .add(diagnostics::plugin)
            .add(shell::plugin)
    }
}
