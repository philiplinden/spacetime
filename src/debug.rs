use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::components::clock;

pub struct DebugToolsPlugin;

impl Plugin for DebugToolsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Update, print_clock_time);
    }
}

fn print_clock_time(query: Query<(Entity, &clock::Clock)>) {
    for (entity, clock) in query.iter() {
        info!("Entity {:?} has time {:?}", entity, clock.time.to_rfc3339())
    }
}


fn spawn_debug_level(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
}

pub struct DebugScenePlugin;

impl Plugin for DebugScenePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_systems(Startup, spawn_debug_level);
    }
}
