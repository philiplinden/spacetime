pub mod orbit_prediction;
pub mod physics;

// The physics engine runs at a fixed timestep, with each step corresponding to the duration specified below.
pub const DT: f32 = 1.0 / 60.0;

// This affects the size of every elements in the physics engine, by multiplying all the length-related quantities by
// the physics_scale factor. This should likely always be 1.0 in 3D. In 2D, this is useful to specify a
// “pixels-per-meter” conversion ratio.
pub static SCALED_LENGTH: f32 = 1.0e-6; // km
pub static SCALED_MASS: f32 = 1.0e-6;  // kg
