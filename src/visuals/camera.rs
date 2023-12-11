use bevy::prelude::*;
use bevy_mouse_tracking_plugin::prelude::MousePosPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanOrbitCameraPlugin)
            .add_plugins(MousePosPlugin)
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.0E5, 1.0E9).looking_at(Vec3::ZERO, Vec3::Z),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}
