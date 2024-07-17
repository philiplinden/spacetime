use anise::prelude::*;
use bevy::prelude::*;

#[derive(Resource)] // should this be a FromWorld impl instead of a resource? I think so. Almanac and Epoch should both be separate resources here, and the "Default" impl below should be a FromWorld
struct SimulationSpace {
    almanac: Almanac,
    epoch: Epoch,
}

impl Default for SimulationSpace {
    fn default() -> SimulationSpace {
        SimulationSpace {
            almanac: match MetaAlmanac::latest() {
                Ok(almanac) => almanac,
                Err(error) => panic!("Problem setting up Almanac: {:?}", error),
            },
            epoch: Epoch::now(),
        }
    }
}
