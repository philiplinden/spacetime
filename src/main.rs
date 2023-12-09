/*!
 * Uses Bevy game engine as the basis for a model of heterogeneous PNT nodes.
 */

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin};
use bevy::time::FixedTimestep;
use bevy::{prelude::*, window::PresentMode};
use bevy_mouse_tracking_plugin::{prelude::*, MainCamera, MousePosWorld};
use bevy_pancam::{PanCam, PanCamPlugin};
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier2d::prelude::*;

const DT: f32 = 1.0 / 60.0;

fn main() {
    // Creating an app, adding systems, and calling .run() handles the exec loop
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: WindowDescriptor {
            title: "Particular demo".to_string(),
            width: 1500.0,
            height: 900.0,
            present_mode: PresentMode::AutoNoVsync,
            fit_canvas_to_parent: true,
            canvas: Some("#app".to_owned()),
            ..default()
        },
        ..default()
    }))
    .add_plugins(FrameTimeDiagnosticsPlugin)
    .add_plugins(EguiPlugin)
    .add_plugins(PanCamPlugin)
    .add_plugins(MousePosPlugin)
    .add_plugins(
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0)
            .with_default_system_setup(false),
    )
    .insert_resource(ClearColor(Color::BLACK))
}
