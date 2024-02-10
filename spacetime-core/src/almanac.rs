use anise::constants::frames::{EARTH_J2000, EARTH_MOON_BARYCENTER_J2000, MOON_J2000};
use anise::prelude::*;

pub fn load_almanac() -> Almanac {
    match MetaAlmanac::latest() {
        Ok(almanac) => almanac,
        Err(error) => panic!("Problem setting up Almanac: {:?}", error),
    }
}

pub fn init_earth_moon_system(almanac: Almanac, epoch: Epoch) -> (Orbit, Orbit) {
    // Earth is at the center of the EARTH_J2000 frame
    let earth_j2k_frame = almanac.frame_from_uid(EARTH_J2000).unwrap();
    let earth = Orbit::zero_at_epoch(epoch, earth_j2k_frame);
    // Moon is at the center of the MOON_J2000 frame
    let moon_j2k_frame = almanac.frame_from_uid(MOON_J2000).unwrap();
    let moon = Orbit::zero_at_epoch(epoch, moon_j2k_frame);

    // Now we transform earth and moon into a common frame about the barycenter
    let barycenter_j2k_frame = almanac.frame_from_uid(EARTH_MOON_BARYCENTER_J2000).unwrap();
    (
        almanac
            .transform_to(earth, barycenter_j2k_frame, None)
            .unwrap(),
        almanac
            .transform_to(moon, barycenter_j2k_frame, None)
            .unwrap(),
    )
}
