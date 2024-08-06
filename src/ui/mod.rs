mod camera;
mod cursor;
mod diagnostics;
mod shell;

use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct UserInterfacePlugins;

impl PluginGroup for UserInterfacePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(bevy_egui::EguiPlugin)
            .add(camera::plugin)
            .add(shell::plugin)
            .add(cursor::plugin)
            .add(diagnostics::plugin)
    }
}
