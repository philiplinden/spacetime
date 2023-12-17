use bevy::prelude::*;
use bevy_mouse_tracking_plugin::prelude::MousePosPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((PanOrbitCameraPlugin, MousePosPlugin))
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, followed_set_parent_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 100.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Z),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}
