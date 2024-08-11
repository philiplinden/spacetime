mod camera;
mod cursor;
mod datepicker;
mod diagnostics;
mod shell;

use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct UserInterfacePlugins;

impl PluginGroup for UserInterfacePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(bevy_egui::EguiPlugin)
            .add(camera::plugin)
            .add(cursor::plugin)
            .add(datepicker::plugin)
            .add(diagnostics::plugin)
            .add(shell::plugin)
    }
}
