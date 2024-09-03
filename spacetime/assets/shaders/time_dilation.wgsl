@group(0) @binding(0) var<uniform> u_params: TimeDilationParams;

struct TimeDilationParams {
    u_time: f32; // Elapsed time
    u_G: f32; // Gravitational constant
    u_M: f32; // Mass of the Moon
    u_c: f32; // Speed of light
    u_v: f32; // Velocity of the satellite
};

@fragment
fn main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    let r = length(pos.xy); // Distance from the Moon's center
    let grav_dilation = sqrt(1.0 - (2.0 * u_params.u_G * u_params.u_M) / (r * u_params.u_c * u_params.u_c));
    let vel_dilation = sqrt(1.0 - (u_params.u_v * u_params.u_v) / (u_params.u_c * u_params.u_c));
    let total_dilation = grav_dilation * vel_dilation;

    // Map the time dilation factor to color
    let color = vec3<f32>(1.0 - total_dilation, 0.0, total_dilation);
    return vec4<f32>(color, 1.0);
}
