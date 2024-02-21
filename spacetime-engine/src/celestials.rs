use anise::constants::frames::{EARTH_J2000, MOON_J2000};
use anise::math::cartesian::CartesianState;
use anise::prelude::*;
use bevy::prelude::*;

use crate::sim::SimulationSpace;

trait Lookup {
    fn true_state(&self, sim: Res<SimulationSpace>, coordinate_frame: Frame) -> CartesianState {
        sim.almanac
            .transform(self.own_frame, coordinate_frame, sim.epoch, None)
            .unwrap()
    }
}

struct CelestialBody {
    own_frame: Frame,
}

impl Lookup for CelestialBody {}

pub static EARTH: CelestialBody = CelestialBody {
    own_frame: EARTH_J2000,
};

pub static LUNA: CelestialBody = CelestialBody {
    own_frame: MOON_J2000,
};
