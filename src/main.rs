/*!
 * Uses Bevy game engine as the basis for a model of heterogeneous PNT nodes.
 */
mod components;
mod cosmic;
mod debug;
mod spacecraft;
mod visuals;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // ui
        .add_plugins((
            visuals::camera::CameraPlugin,
            visuals::ui::FpsCounterPlugin,
        )) // physics
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // spawn entities
        // .add_plugins(cosmic::InitCelestialsPlugin)
        .add_plugins(spacecraft::InitParticipantsPlugin)
        // debug
        .add_plugins(debug::DebugToolsPlugin)
        .run();
}
