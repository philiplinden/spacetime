// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;
mod model;
mod ui;
mod scenes;

fn main() -> AppExit {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Window {
                title: "💫 spacetime".to_string(),
                ..default()
            }
            .into(),
            ..default()
        }),
    ));

    app.add_plugins((
        ui::plugin,
        model::plugin,
        scenes::plugin,
    ));

    app.run()
}
