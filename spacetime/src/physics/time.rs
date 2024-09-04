use std::fmt;

use avian2d::schedule::Physics;
use bevy::prelude::*;
use hifitime::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<CoordinateTime>();
    app.add_systems(Update, sync_coordinate_time);
}

/// Coordinate Time is the clock belonging to the "external" observer, in this case the game world.
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
            Err(_) => Epoch::from_utc_duration(zero_seconds),
        };
        CoordinateTime {
            scale: TimeScale::UTC,
            elapsed: zero_seconds,
            start_epoch: Some(start_epoch),
        }
    }
}

impl CoordinateTime {
    /// The current epoch (starting epoch + elapsed system time)
    pub fn epoch(&self) -> Epoch {
        self.start_epoch
            .unwrap_or_else(|| Epoch::from_duration(Duration::ZERO, self.scale))
            + self.elapsed
    }

    /// The current epoch, represented in some other timescale. Might be useless
    #[allow(dead_code)]
    pub fn epoch_in_scale(&self, time_scale: TimeScale) -> Epoch {
        let native_epoch = self.start_epoch.unwrap_or_default() + self.elapsed;
        native_epoch.in_time_scale(time_scale)
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
            "{:} ({:.4} seconds since {:.4})",
            self.epoch(),
            self.elapsed_seconds(),
            self.start_epoch.unwrap_or_default()
        )
    }
}

/// Increment the elapsed coordinate time by adding the time since the last tick
///
/// Avian Physics Time is used as the Coordinate Time but this system runs every
/// update. That means the actual time updates with the physics schedule, and
/// the coordinate time resource is always as up to date as possible.
fn sync_coordinate_time(
    physics_time: Res<Time<Physics>>,
    mut coordinate_time: ResMut<CoordinateTime>,
) {
    coordinate_time.elapsed = coordinate_time.elapsed
        + Duration::from_f64(physics_time.delta_seconds_f64(), Unit::Second);
}
