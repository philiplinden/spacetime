use bevy::prelude::*;
mod components;
mod gui;
mod scenario;
mod physics;

use components::{body, camera};

// The physics engine runs at a fixed timestep, with each step corresponding to the duration specified below.
pub const DT: f32 = 1.0 / 60.0;

// This affects the size of every elements in the physics engine, by multiplying all the length-related quantities by
// the physics_scale factor. This should likely always be 1.0 in 3D. In 2D, this is useful to specify a
// “pixels-per-meter” conversion ratio.
pub static SCALED_LENGTH: f32 = 1.0e-6; // km
pub static SCALED_MASS: f32 = 1.0e-6;  // kg

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
            physics::PhysicsSchedulePlugin,
            physics::GravityPlugin,
            // Scene
            scenario::ScenarioPlugin,
        ))
        // Resources
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.5,
        })
        .insert_resource(physics::PhysicsSettings::delta_time(DT))
        .add_systems(First, body::add_materials)
        .run();
}
