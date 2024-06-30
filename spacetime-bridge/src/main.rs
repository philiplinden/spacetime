use bevy::prelude::*;
mod gui;
mod scenario;
mod scene;

use scene::{body, camera};
use spacetime_core::{kinematics, schedule, PhysicsSettings, G, SCALE};

use gui::select::{Followed, Selected};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    #[cfg(not(target_arch = "wasm32"))]
                    resolution: bevy::window::WindowResolution::new(600.0, 400.0),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    canvas: Some("#app".to_owned()),
                    ..default()
                }),
                ..default()
            }),
            // Interface
            gui::GuiPlugin,
            camera::CameraPlugin,
            // Physics
            schedule::PhysicsSchedulePlugin,
            kinematics::GravityPlugin,
            // Scene
            scenario::ScenarioPlugin,
        ))
        // Resources
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.5,
        })
        .insert_resource(PhysicsSettings::delta_time(DT))
        .add_systems(First, body::add_materials)
        .run();
}
