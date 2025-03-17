use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub struct DemoScenePlugin;

impl Plugin for DemoScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK));
        app.add_systems(Startup, (setup, setup_camera, setup_lighting));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sphere_handle = meshes.add(Sphere::new(1.0));
    let sphere_material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.7, 0.6),
        ..default()
    });

    commands
        .spawn((
            Mesh3d(sphere_handle.clone()),
            MeshMaterial3d(sphere_material_handle.clone()),
            Transform::from_xyz(0.0, 0.0, 1.0),
        ));
}

#[derive(Component)]
struct ViewportCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        OrthographicProjection {
            scale: 10.0,
            near: -1000.0,
            far: 1000.0,
            scaling_mode: ScalingMode::FixedVertical { viewport_height: 10.0 },
            ..OrthographicProjection::default_3d()
        },
        ViewportCamera,
        Transform::from_xyz(0.0, 0.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn setup_lighting(mut commands: Commands) {
    commands.spawn((PointLight::default(), Transform::from_xyz(4.0, 5.0, 4.0)));
}
