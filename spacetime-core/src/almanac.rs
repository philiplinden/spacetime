use anise::constants::frames::{EARTH_J2000, EARTH_MOON_BARYCENTER_J2000, MOON_J2000};
use anise::math::cartesian::CartesianState;
use anise::prelude::*;

pub fn load_almanac() -> Almanac {
    match MetaAlmanac::latest() {
        Ok(almanac) => almanac,
        Err(error) => panic!("Problem setting up Almanac: {:?}", error),
    }
}

pub fn init_earth_moon_system(almanac: Almanac, epoch: Epoch) -> (CartesianState, CartesianState) {
    let earth = almanac.transform(EARTH_J2000, EARTH_MOON_BARYCENTER_J2000, epoch, None).unwrap();
    let moon = almanac.transform(MOON_J2000, EARTH_MOON_BARYCENTER_J2000, epoch, None).unwrap();
    (earth, moon)
}
