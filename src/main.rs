/*!
 * Uses Bevy game engine as the basis for a model of heterogeneous PNT nodes.
 */
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod components;
mod debug;
mod visuals;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // ui
        .add_plugins(visuals::ui::FpsCounterPlugin)
        .add_plugins(visuals::camera::CameraPlugin)
        // physics
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // debug
        .add_plugins(debug::DebugToolsPlugin)
        .add_plugins(debug::DebugScenePlugin)
        .run();
}
