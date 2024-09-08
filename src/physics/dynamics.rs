use avian2d::prelude::*;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use particular::prelude::*;

pub const COMPUTE_METHOD: parallel::BruteForceScalar = parallel::BruteForceScalar;
const G: f32 = 1000.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_bodies);
    app.add_systems(
        PhysicsSchedule,
        (rotate_spinners, accelerate_bodies).in_set(PhysicsStepSet::First),
    );
}

/// Marker component for bodies that we want to spin like gears.
#[derive(Component)]
struct Spinner;

/// Rotates the center body periodically clockwise and counterclockwise.
fn rotate_spinners(
    mut query: Query<&mut AngularVelocity, With<Spinner>>,
    time: Res<Time<Physics>>,
) {
    let sin = 5.0 * time.elapsed_seconds().sin();
    for mut ang_vel in &mut query {
        ang_vel.0 = sin;
    }
}

/// Accelerate massive bodies with n-body physics. This only updates the linear
/// velocity of each body---Avian physics handles the rest.
fn accelerate_bodies(
    time: Res<Time<Physics>>,
    mut bodies: Query<(&mut LinearVelocity, &Position, &Mass)>,
) {
    let delta_time = time.delta_seconds();
    let accelerations = bodies
        .iter()
        .map(|(_velocity, position, mass)| (position.to_array(), mass.0 * G))
        .accelerations(&mut COMPUTE_METHOD.clone());
    for (acceleration, (mut velocity, ..)) in accelerations.zip(&mut bodies) {
        let delta_velocity = Vec2::from(acceleration) * delta_time;
        let new_velocity = velocity.0 + delta_velocity;
        velocity.0 = new_velocity;
    }
}

fn spawn_bodies(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let center_radius = 200.0;
    let particle_radius = 5.0;

    let red = materials.add(Color::srgb(0.9, 0.3, 0.3));
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
            Spinner,
        ))
        .with_children(|c| {
            // Spawn obstacles along the perimeter of the rotating body, like the teeth of a cog.
            let count = 8;
            let angle_step = std::f32::consts::TAU / count as f32;
            for i in 0..count {
                let pos = Quat::from_rotation_z(i as f32 * angle_step) * Vec3::Y * center_radius;
                c.spawn((
                    MaterialMesh2dBundle {
                        mesh: particle_mesh.clone().into(),
                        material: red.clone(),
                        transform: Transform::from_translation(pos).with_scale(Vec3::ONE * 5.0),
                        ..default()
                    },
                    Collider::circle(particle_radius),
                    Mass(1_000_000.0),
                ));
            }
        });

    let x_count = 10;
    let y_count = 30;

    // Spawn grid of particles. These will be pulled towards the rotating body.
    for x in -x_count / 2..x_count / 2 {
        for y in -y_count / 2..y_count / 2 {
            let pos: Vec2 = Vec2::new(x as f32 * 3.0 * particle_radius - 350.0,
            y as f32 * 3.0 * particle_radius);
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
            ));
        }
    }
}
