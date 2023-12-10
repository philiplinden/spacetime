use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::cosmic::EARTH_RADIUS_M;

pub struct InitEntitiesPlugin;

impl Plugin for InitEntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball);
    }
}

fn spawn_ball(mut commands: Commands) {
    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, EARTH_RADIUS_M, 0.0)));
}
