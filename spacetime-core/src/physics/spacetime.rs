/*!
 * This module handles the warping of spacetime. This includes gravity and time dilation.
 *
 */

use bevy::prelude::*;
use particular::prelude::*;

use crate::physics::schedule::{PhysicsSchedule, PhysicsSet};

// from particular::prelude
pub const NBODY_COMPUTE_METHOD: sequential::BruteForcePairs = sequential::BruteForcePairs;

#[derive(Component, Clone, Copy, Default, Deref, DerefMut, Reflect)]
pub struct Position(pub Vec3);

#[derive(Component, Clone, Copy, Default, Deref, DerefMut, Reflect)]
pub struct Velocity(pub Vec3);

#[derive(Component, Clone, Copy, Default, Deref, DerefMut, Reflect)]
pub struct Acceleration(pub Vec3);

#[derive(Component, Clone, Copy, Default, Deref, DerefMut)]
pub struct Mass(pub f32);

pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PhysicsSchedule,
            accelerate_particles.in_set(PhysicsSet::First),
        );
    }
}

pub fn sympletic_euler(
    acceleration: Vec3,
    mut velocity: Vec3,
    mut position: Vec3,
    dt: f32,
) -> (Vec3, Vec3) {
    velocity += acceleration * dt;
    position += velocity * dt;

    (velocity, position)
}

fn accelerate_particles(mut query: Query<(&mut Acceleration, &Position, &Mass)>) {
    query
        .iter()
        .map(|(.., position, mass)| (position.to_array(), **mass))
        .accelerations(&mut NBODY_COMPUTE_METHOD.clone())
        .map(Vec3::from)
        .zip(&mut query)
        .for_each(|(acceleration, (mut physics_acceleration, ..))| {
            **physics_acceleration = acceleration
        });
}
/*
 * The time dilation of an accelerating object is given by
 *
 *      tau(t) = (c / g) arcsinh(g * t / c), where
 *
 *          t -> elapsed proper time
 *          c -> speed of light
 *          g -> acceleration
 *
 * For other relativistic effects, we may need to reference the Lorentz factor given by
 *
 *      gamma = 1 / sqrt(1 - v^2 / c^2), where
 *
 *          v -> speed of moving observer
 *          c -> speed of light
 *
 * Time dilation can also be expressed in terms of Lorentz factor:
 *
 *      dt = gamma * dtau
 *
 * Note that if we integrate over time and substitute v -> (g * t), we arrive at the same definition as before. However,
 * using the more complicated form with acceleration lets us account for the accelerations from gravity when an object
 * is near a large body.
 */