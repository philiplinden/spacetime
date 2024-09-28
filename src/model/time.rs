use std::fmt;

use bevy::prelude::*;
use hifitime::{Duration, Epoch, TimeScale};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource::<CoordinateTime>(CoordinateTime::new(TimeScale::UTC, None))
        .register_type::<Clock>()
        .add_systems(Update, update_clock)
        .add_systems(Update, update_simulation_speed);
}

/// A scalar factor to speed up or slow down the simulation.
/// A value of 1.0 will advance the world clock at the same rate as real time.
/// Values greater than 1.0 speed up the simulation, while values less than 1.0
/// slow it down.
#[derive(Resource, Clone, Copy, Reflect)]
#[reflect(Resource)]
pub struct SimulationSpeed {
    pub factor: f64,
}

impl Default for SimulationSpeed {
    fn default() -> Self {
        Self { factor: 1.0 }
    }
}

impl SimulationSpeed {
    pub fn new(factor: f64) -> Self {
        Self { factor }
    }
}

/// Coordinate Time is the clock belonging to the "external" observer, in this
/// case the game world.
#[derive(Resource, Clone, Copy)]
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

/// A struct representing a unique clock attached to an entity.
/// The proper time is the time experienced by the clock, and the coordinate
/// time is the time experienced by an observer at rest with respect to the
/// clock.
#[derive(Component, Debug, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct Clock {
    pub proper_time: f64,
    pub coordinate_time: f64,
}

impl Default for Clock {
    fn default() -> Self {
        Clock {
            proper_time: 0.0,
            coordinate_time: 0.0,
        }
    }
}

fn update_clock(time: Res<Time<Virtual>>, mut clock_query: Query<&mut Clock>) {
    if let Ok(mut clock) = clock_query.get_single_mut() {
        // Update proper time and coordinate time using the engine's time
        clock.proper_time += time.delta_seconds_f64();
        clock.coordinate_time += time.delta_seconds_f64();
    }
}
fn update_simulation_speed(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut time: ResMut<Time<Virtual>>,
) {
    let speed_change = if keyboard_input.pressed(KeyCode::Period) {
        0.1
    } else if keyboard_input.pressed(KeyCode::Comma) {
        -0.1
    } else if keyboard_input.pressed(KeyCode::Slash) {
        1.0 - time.relative_speed()
    } else {
        0.0
    };

    if speed_change != 0.0 {
        let new_speed = (time.relative_speed() + speed_change).max(0.1).min(10.0);
        time.set_relative_speed(new_speed);
        info!("Simulation speed set to {:.1}", new_speed);
    }
}
