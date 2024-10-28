use bevy::prelude::*;
use rand;

use std::fmt;
use std::ops::{Add, AddAssign};

use bevy::prelude::*;
use hifitime::{Duration, Epoch, TimeScale};
use log::error;

use crate::{time::TimeSettings, WorldState};

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TimeSettings>()
            .add_plugins((coordinate::CoordinateTimePlugin, clock::ClockPlugin));
    }
}

/// A collection of settings that control the time simulation. `speed` is a
/// scalar factor sets the ratio of simulation time advancement relative to real
/// time. At 1.0, simulation time advances at system time.
#[derive(Resource, Default, Reflect)]
pub struct TimeSettings {
    speed: f32,
}

impl Default for TimeSettings {
    fn default() -> Self {
        TimeSettings { speed: 1.0 }
    }
}

pub struct CoordinateTimePlugin;

impl Plugin for CoordinateTimePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CoordinateTime>()
            .init_resource::<CoordinateTime>()
            .add_systems(
                Update,
                advance_coordinate_time.run_if(in_state(WorldState::Running)),
            )
    }
}

/// Coordinate Time is the clock belonging to the "external" observer, in this
/// case the game world.
#[derive(Clone, Copy)]
pub struct CoordinateTime {
    pub scale: TimeScale,
    pub elapsed: Duration,
    pub start_epoch: Option<Epoch>,
}

impl Default for CoordinateTime {
    fn default() -> Self {
        let zero_seconds = Duration::from_seconds(0.0);
        let start_epoch = match Epoch::now() {
            Ok(value) => value,
            Err(_) => {
                error!("Error getting the current time! Using epoch origin");
                Epoch::from_utc_duration(zero_seconds)
            }
        };
        CoordinateTime {
            scale: TimeScale::UTC,
            elapsed: zero_seconds,
            start_epoch: Some(start_epoch),
        }
    }
}

#[allow(dead_code)]
impl CoordinateTime {
    pub fn new(scale: TimeScale, start_epoch: Option<Epoch>) -> Self {
        let zero_seconds = Duration::from_seconds(0.0);
        CoordinateTime {
            scale,
            elapsed: zero_seconds,
            start_epoch: Some(start_epoch.unwrap_or_default()),
        }
    }

    /// Get the current epoch (start_epoch + elapsed time). It also casts to the
    /// currently set time scale just in case it was changed.
    pub fn epoch(&self) -> Epoch {
        self.start_epoch.unwrap_or_default() + self.elapsed
    }

    /// Number of seconds of system time that have elapsed since the start
    /// epoch. This is here for convenience for when we don't want to clutter
    /// code converting from hifitime::Durations.
    pub fn elapsed_seconds(&self) -> f64 {
        self.elapsed.to_seconds()
    }

    /// Reset the coordinate time to the start epoch.
    pub fn reset(&mut self) -> Self {
        CoordinateTime {
            scale: self.scale,
            elapsed: Duration::from_seconds(0.0),
            start_epoch: self.start_epoch,
        }
    }
}

impl Add<Duration> for CoordinateTime {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self::Output {
        CoordinateTime {
            scale: self.scale,
            elapsed: self.elapsed + rhs,
            start_epoch: self.start_epoch,
        }
    }
}

impl AddAssign<Duration> for CoordinateTime {
    fn add_assign(&mut self, rhs: Duration) {
        self.elapsed += rhs;
    }
}

impl fmt::Display for CoordinateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.epoch())
    }
}

impl fmt::Debug for CoordinateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:} ({:.4} seconds since {:} epoch)",
            self.epoch(),
            self.scale,
            self.elapsed_seconds(),
        )
    }
}

fn advance_coordinate_time(
    mut coordinate_time: RexMut<CoordinateTime>,
    system_time: Res<Time<Virtual>>,
    sim_speed: Res<TimeSettings>,
) {
    let system_delta = Duration::from_seconds(system_time.delta_seconds_f64());
    coordinate_time += system_delta;
}

pub struct ClockPlugin;

impl Plugin for ClockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            tick_all_clocks.run_if(in_state(WorldState::Running)),
        )
    }
}

pub enum ClockType {
    /// A clock that ticks at a constant rate with no variation or drift.
    Ideal,
    /// A clock that ticks with some random variation and drift.
    Real,
    // TODO: add more nuanced types of real clocks instead of just one.
}

/// The `Clock` struct is a generic type for timekeeping components. It allows
/// for the creation of various clock models with different behaviors, such as
/// ideal clocks, real clocks with drift, or clocks affected by relativistic
/// effects.Time is always measured as the proper time, which may diverge from
/// the simulation's coordinate time.
///
/// This component is designed to provide a common interface for all clock
/// types, enabling easy interchangeability and extensibility of clock behaviors
/// in the simulation. It allows for modeling of time-related phenomena and
/// their effects on different objects or agents in the simulated world.
#[derive(Component, Debug, Clone, Copy)]
pub struct Clock {
    pub clock_type: ClockType,
    pub proper_time: Epoch,
    frequency_drift: f32,
    random_walk_chance: f32,
}

impl Clock {
    fn tick(&mut self, delta_time: Duration) {
        let tick_delta = match self.clock_type {
            ClockType::Ideal => delta_time,
            ClockType::Real => real_tick(delta_time),
        };
        self.proper_time += tick_delta;
    }
}

fn real_tick(delta_time: Duration) -> Duration {
    let mut rng = rand::thread_rng();
    let variation = rng.gen_range(-0.1..0.1);
    delta_time * (1.0 + variation)
}

fn tick_all_clocks(mut clocks: Query<Clock>) {
    for clock in clocks.itermut() {
        clock.tick();
    }
}
