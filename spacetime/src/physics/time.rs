use std::fmt;

use avian2d::schedule::Physics;
use bevy::prelude::*;
use hifitime::{Duration, Epoch, TimeScale};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource::<CoordinateTime>(CoordinateTime::new(TimeScale::UTC, None));
    app.add_systems(Update, sync_coordinate_time);
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

    /// Map the elapsed time to a convenience function that follows hifitime
    /// patterns and returns a simple float rather than a Duration type.
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

/// Avian Physics Time is used as the Coordinate Time
fn sync_coordinate_time(
    physics_time: Res<Time<Physics>>,
    mut coordinate_time: ResMut<CoordinateTime>,
) {
    coordinate_time.elapsed =
        coordinate_time.elapsed + Duration::from_seconds(physics_time.delta_seconds_f64());
}
