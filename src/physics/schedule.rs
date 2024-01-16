/* This is a custom physics schedule for use with Bevy.
 * It is based on https://github.com/Canleskis/particular/blob/main/examples/pocket-solar-system/src/physics.rs
 *
 * This is basically a tiny physics engine. Unlike fully-feature physics engines like Rapier, this physics schedule
 * ONLY calculates motion. It does NOT include features related to collisions.
 */

use std::time::Duration;

use bevy::ecs::schedule;
use bevy::prelude::*;

use crate::physics::{sympletic_euler, Acceleration, Velocity, Position};

#[derive(schedule::ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PhysicsSchedule;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum PhysicsSet {
    First,
    Main,
    Last,
}

#[derive(Resource, Clone, Copy)]
pub struct PhysicsSettings {
    pub delta_time: f32,
    pub time_scale: f32,
}

impl PhysicsSettings {
    pub fn delta_time(delta_time: f32) -> Self {
        Self {
            delta_time,
            ..default()
        }
    }

    pub fn period(&self) -> f32 {
        self.delta_time / self.time_scale
    }

    pub fn steps_per_second(&self) -> usize {
        self.delta_time.recip().round() as _
    }
}

impl Default for PhysicsSettings {
    fn default() -> Self {
        Self {
            delta_time: 1.0 / 60.0,
            time_scale: 1.0,
        }
    }
}

#[derive(Resource, Default)]
pub struct PhysicsTime {
    accumulated: f32,
    pub paused: bool,
}

impl PhysicsTime {
    fn tick(&mut self, delta: f32) {
        if !self.paused {
            self.accumulated += delta;
        }
    }

    fn can_step(&self, period: f32) -> bool {
        !self.paused && self.accumulated >= period
    }
}

#[derive(Resource, Deref, Clone, Copy, Default)]
pub struct ElapsedPhysicsTime(Duration);

#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct Interpolated {
    previous_position: Option<Vec3>,
}

pub struct PhysicsSchedulePlugin;

impl Plugin for PhysicsSchedulePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Acceleration>()
            .register_type::<Velocity>()
            .add_schedule(Schedule::new(PhysicsSchedule))
            .configure_sets(
                PhysicsSchedule,
                (PhysicsSet::First, PhysicsSet::Main, PhysicsSet::Last).chain(),
            )
            .insert_resource(PhysicsTime::default())
            .insert_resource(ElapsedPhysicsTime::default())
            .add_systems(PreUpdate, run_physics_schedule)
            .add_systems(
                PhysicsSchedule,
                (
                    (track_elapsed_time, cache_previous_positions).in_set(PhysicsSet::First),
                    integrate_positions.in_set(PhysicsSet::Main),
                ),
            )
            .add_systems(Update, update_transforms);
    }
}

fn run_physics_schedule(world: &mut World) {
    let delta = world.resource::<Time>().delta_seconds();
    world.resource_mut::<PhysicsTime>().tick(delta);

    let period = world.resource::<PhysicsSettings>().period();
    while world.resource::<PhysicsTime>().can_step(period) {
        world.resource_mut::<PhysicsTime>().accumulated -= period;
        world.run_schedule(PhysicsSchedule);
    }
}

fn track_elapsed_time(physics: Res<PhysicsSettings>, mut elapsed: ResMut<ElapsedPhysicsTime>) {
    elapsed.0 += Duration::from_secs_f32(physics.delta_time);
}

fn cache_previous_positions(mut query: Query<(&Position, &mut Interpolated)>) {
    for (position, mut interpolated) in &mut query {
        interpolated.previous_position = Some(**position);
    }
}

fn integrate_positions(
    physics: Res<PhysicsSettings>,
    mut query: Query<(&mut Acceleration, &mut Velocity, &mut Position)>,
) {
    for (mut acceleration, mut velocity, mut position) in &mut query {
        (**velocity, **position) =
            sympletic_euler(**acceleration, **velocity, **position, physics.delta_time);

        **acceleration = Vec3::ZERO;
    }
}

/// Interpolates or sets the transform depending on if the entity has an [`Interpolated`] component.
/// Can look unnatural if the `delta_time / time_scale` change is significant between two frames.
fn update_transforms(
    physics: Res<PhysicsSettings>,
    physics_time: Res<PhysicsTime>,
    mut query: Query<(&mut Transform, &Position, Option<&Interpolated>)>,
) {
    if physics_time.paused {
        return;
    }

    let s = physics_time.accumulated / physics.period();

    for (mut transform, position, interpolated) in &mut query {
        let new_position = interpolated
            .and_then(|interpolated| interpolated.previous_position)
            .map(|previous_position| previous_position.lerp(**position, s))
            .unwrap_or(**position);

        if transform.translation != new_position {
            transform.translation = new_position;
        }
    }
}
