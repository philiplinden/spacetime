use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod components;
mod gui;
mod physics;

use components::{body, camera};

pub const DT: f32 = 1.0 / 60.0;

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
            gui::debug::DebugUiPlugin, // requires BaseUiPlugin
            camera::CameraPlugin,

            //Physics
            RapierPhysicsPlugin::<NoUserData>::default().with_default_system_setup(false),
            physics::CustomRapierSchedule,
        ))
        // Spawn bodies
        .add_systems(Startup, body::spawn_bodies)
        .add_systems(First, body::add_materials)
        .run();
}

// fn load_gltf_things(mut commands: Commands, server: Res<AssetServer>) {
//     // spawn a whole scene
//     let my_scene: Handle<Scene> = server.load("models/Earth.glb#Scene0");
//     commands.spawn(SceneBundle {
//         scene: my_scene,
//         ..Default::default()
//     });
// }
