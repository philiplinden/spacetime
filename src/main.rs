mod components;
mod gui;

use bevy::prelude::*;

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
            gui::BaseUiPlugin,
            gui::DebugUiPlugin, // requires BaseUiPlugin
        ))
        .add_systems(Startup, load_gltf_things)
        .run();
}

fn load_gltf_things(
    mut commands: Commands,
    server: Res<AssetServer>
) {
    // spawn a whole scene
    let my_scene: Handle<Scene> = server.load("models/Earth.gltf#Scene0");
    commands.spawn(SceneBundle {
        scene: my_scene,
        ..Default::default()
    });
}