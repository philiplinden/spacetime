use std::fmt;
use std::ops::{Add, AddAssign};

use log::error;
use hifitime::{Duration, Epoch, TimeScale};
use krabmaga::rand::{self, Rng};

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

/// A struct representing a clock that ticks at a constant rate.
/// The proper time is the time experienced by the clock, which may diverge from
/// the simulation's coordinate time.
#[derive(Debug, Clone, Copy)]
pub struct Clock {
    pub proper_time: Epoch,
}

/// The `ClockBehavior` trait defines the interface for different types of
/// clocks in the simulation. It allows for the creation of various clock models
/// with different behaviors, such as ideal clocks, real clocks with drift, or
/// clocks affected by relativistic effects.
///
/// This trait exists to provide a common interface for all clock types,
/// enabling easy interchangeability and extensibility of clock behaviors in the
/// simulation. It allows for modeling of time-related phenomena and their
/// effects on different objects or agents in the simulated world.
pub trait ClockBehavior: Send + Sync + Clone {
    fn new(coordinate_time: CoordinateTime) -> Self
    where
        Self: Sized;

    fn tick(&mut self, delta_time: Duration);

    fn get_time(&self) -> Epoch;

    fn set_time(&mut self, time: Epoch);
}

impl ClockBehavior for Clock {
    /// Create a new clock with the given coordinate time.
    fn new(coordinate_time: CoordinateTime) -> Self {
        Self {
            proper_time: coordinate_time.epoch(),
        }
    }

    /// Tick the clock by the given amount of delta coordinate time.
    fn tick(&mut self, _delta_time: Duration) {}

    fn get_time(&self) -> Epoch {
        self.proper_time
    }

    fn set_time(&mut self, time: Epoch) {
        self.proper_time = time;
    }
}

/// An ideal clock that ticks at a constant rate.
#[derive(Debug, Clone, Copy)]
pub struct IdealClock(Clock);

impl ClockBehavior for IdealClock {
    fn new(coordinate_time: CoordinateTime) -> Self {
        IdealClock(Clock::new(coordinate_time))
    }

    fn tick(&mut self, delta_time: Duration) {
        self.0.tick(delta_time);
    }

    fn get_time(&self) -> Epoch {
        self.0.get_time()
    }

    fn set_time(&mut self, time: Epoch) {
        self.0.set_time(time);
    }
}

/// A clock that ticks at a constant rate, but with some random variation.
#[derive(Debug, Clone, Copy)]
pub struct RealClock(Clock);

impl ClockBehavior for RealClock {
    fn new(coordinate_time: CoordinateTime) -> Self {
        RealClock(Clock::new(coordinate_time))
    }

    fn tick(&mut self, delta_time: Duration) {
        let mut rng = rand::thread_rng();
        let variation = rng.gen_range(-0.1..0.1);
        let adjusted_delta = delta_time * (1.0 + variation);
        self.0.tick(adjusted_delta);
    }

    fn get_time(&self) -> Epoch {
        self.0.get_time()
    }

    fn set_time(&mut self, time: Epoch) {
        self.0.set_time(time);
    }
}
