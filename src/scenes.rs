use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use avian2d::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Startup, spawn_bodies,
    );
}

fn spawn_bodies(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let center_radius = 200.0;
    let particle_radius = 5.0;

    let blue = materials.add(Color::srgb(0.1, 0.6, 1.0));
    let particle_mesh = meshes.add(Circle::new(particle_radius));

    // Spawn rotating body at the center.
    commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(center_radius)).into(),
                material: materials.add(Color::srgb(0.7, 0.7, 0.8)).clone(),
                ..default()
            },
            RigidBody::Kinematic,
            Collider::circle(center_radius),
            Mass(1_000_000.0),
        ));

    let x_count = 10;
    let y_count = 30;

    // Spawn grid of particles. These will be pulled towards the rotating body.
    for x in -x_count / 2..x_count / 2 {
        for y in -y_count / 2..y_count / 2 {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: particle_mesh.clone().into(),
                    material: blue.clone(),
                    transform: Transform::from_xyz(
                        x as f32 * 3.0 * particle_radius - 350.0,
                        y as f32 * 3.0 * particle_radius,
                        0.0,
                    ),
                    ..default()
                },
                RigidBody::Dynamic,
                Collider::circle(particle_radius),
                LinearVelocity(Vec2::new(0.0, 500.0))
            ));
        }
    }
}
