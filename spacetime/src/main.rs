// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;

mod bodies;
mod physics;
mod ui;

fn main() -> AppExit {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Window {
                title: "spacetime".to_string(),
                ..default()
            }
            .into(),
            ..default()
        }),
        physics::plugin,
        bodies::CelestialsPlugin,
        // bodies::SatellitesPlugin,
        ui::UserInterfacePlugins,
    ));
    app.init_state::<AppState>();
    app.run()
}

#[derive(Resource, Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    Splash,
    Loading,
    Paused,
    #[default]
    Running,
    Credits,
}
