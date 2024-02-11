pub mod almanac;
pub mod schedule;
pub mod kinematics;

pub use schedule::{
    ElapsedPhysicsTime, Interpolated, PhysicsSettings, PhysicsTime,
};
pub use kinematics::{
    sympletic_euler, Acceleration, GravityPlugin, Mass, Position, Velocity, NBODY_COMPUTE_METHOD,
};

// Universal gravitational constant in SI units [N m^2 kg^-2]
pub const G: f32 = 6.67430e-11;

// The physics engine runs at a fixed timestep, with each step corresponding to the duration specified below.
pub const DT: f32 = 1.0 / 60.0; // seconds

// This affects the size of every elements in the physics engine, by multiplying all the length-related quantities by
// the physics_scale factor. This should likely always be 1.0 in 3D. In 2D, this is useful to specify a
// “pixels-per-meter” conversion ratio.
pub static SCALE: f32 = 1.0e-6;

#[cfg(test)]
mod tests {
    #[test]
    fn test_load_latest_almanac() {
        // if we don't panic, test is good
        // FIXME(phil): write graceful handling when almanac doesn't load
        let almanac = crate::almanac::load_almanac();
        println!("{:?}", almanac.describe(None, None, None, None, None));
    }
}
