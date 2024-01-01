mod components;
mod ui;

use ui::CustomUiPlugin;

use bevy::prelude::*;

fn main() {
    pretty_env_logger::init();
    App::new()
        .add_plugins((
            DefaultPlugins,
            CustomUiPlugin,
        ))
        .run();
}
