use avian2d::prelude::*;
use bevy::prelude::*;
use particular::prelude::*;

pub const COMPUTE_METHOD: parallel::BruteForceScalar = parallel::BruteForceScalar;
const G: f32 = 100.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        PhysicsSchedule, accelerate_bodies.in_set(PhysicsStepSet::First),
    );
}

/// Accelerate massive bodies with n-body physics. This only updates the linear
/// velocity of each body---Avian handles the integration and updates position.
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
