// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;
mod physics;
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
        physics::plugin,
        scenes::plugin,
    ));

    app.init_state::<AppState>();
    app.run()
}

#[derive(Resource, Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    Splash,
    Loading,
    #[default]
    Paused,
    Running,
}
