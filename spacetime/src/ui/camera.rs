use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PanCamPlugin::default());
    app.add_systems(Startup, init_camera);
}

fn init_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), PanCam::default()));
}
