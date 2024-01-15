use bevy::prelude::*;
mod components;
mod gui;
mod physics;

use components::{body, camera};

// The physics engine runs at a fixed timestep, with each step corresponding to the duration specified below.
pub const DT: f32 = 1.0 / 60.0;

// This affects the size of every elements in the physics engine, by multiplying all the length-related quantities by
// the physics_scale factor. This should likely always be 1.0 in 3D. In 2D, this is useful to specify a
// “pixels-per-meter” conversion ratio.
pub const SCALE: f32 = 1.0;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    #[cfg(not(target_arch = "wasm32"))]
                    resolution: bevy::window::WindowResolution::new(1920.0, 1080.0),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    canvas: Some("#app".to_owned()),
                    ..default()
                }),
                ..default()
            }),
            // Interface
            gui::BaseUiPlugin,
            gui::hud::HudPlugin,
            gui::orbit_prediction::OrbitPredictionPlugin,
            gui::select::SelectionPlugin,
            gui::debug::DebugUiPlugin, // requires BaseUiPlugin
            // Camera
            camera::CameraPlugin,
            //Physics
            physics::PhysicsPlugin,
            physics::GravityPlugin,
        ))
        // Resources
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight {
            color: Color::NONE,
            brightness: 0.0,
        })
        .insert_resource(physics::PhysicsSettings::delta_time(1.0 / 60.0))
        // Spawn bodies
        .add_systems(Startup, body::setup_scene)
        .add_systems(First, body::add_materials)
        .run();
}
