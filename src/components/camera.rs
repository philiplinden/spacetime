use bevy::{
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    prelude::*,
};

use crate::SCALE;

static MAX_ZOOM_LIMIT: f32 = 1.0e3;

#[derive(Component, Default)]
pub struct OrbitCamera {
    pub min_distance: f32,
    pub focus: Vec3,
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(PostUpdate, camera_controls);
    }
}

pub fn camera_controls(
    query_windows: Query<&Window>,
    input_mouse: Res<Input<MouseButton>>,
    mut scroll_events: EventReader<MouseWheel>,
    mut motion_events: EventReader<MouseMotion>,
    mut query_camera: Query<(&mut Transform, &OrbitCamera)>,
    mut radius: Local<Option<f32>>,
) {
    let radius = radius.get_or_insert(500.0);

    let Ok((mut transform, orbit)) = query_camera.get_single_mut() else {
        return;
    };
    let Ok(window) = query_windows.get_single() else {
        return;
    };

    let window_size = Vec2::new(window.width(), window.height());

    let scroll = scroll_events
        .read()
        .map(|ev| match ev.unit {
            MouseScrollUnit::Pixel => ev.y * 0.005,
            MouseScrollUnit::Line => ev.y * 1.0,
        })
        .sum::<f32>();

    *radius -= *radius * scroll * 0.2;
    *radius = radius.clamp(orbit.min_distance, MAX_ZOOM_LIMIT);

    let delta = input_mouse
        .pressed(MouseButton::Right)
        .then(|| motion_events.read().map(|ev| ev.delta).sum::<Vec2>())
        .unwrap_or_default()
        / window_size
        * std::f32::consts::PI;
    motion_events.clear();

    let Transform {
        mut rotation,
        scale,
        ..
    } = *transform;

    rotation *= Quat::from_rotation_y(-delta.x * 2.0);
    rotation *= Quat::from_rotation_x(-delta.y);

    let translation =
        orbit.focus + Mat3::from_quat(rotation).mul_vec3(Vec3::new(0.0, 0.0, *radius));

    *transform = Transform {
        translation,
        rotation,
        scale,
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1.0e7 * SCALE)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        },
        OrbitCamera {
            min_distance: 2.0,
            focus: Vec3::ZERO,
        },
        bevy::core_pipeline::bloom::BloomSettings {
            intensity: 0.15,
            ..default()
        },
    ));
}
