// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

mod scene;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            spacetime::SpacetimePlugin,
            scene::DemoScenePlugin,
        ))
        .run();
}
