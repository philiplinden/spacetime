@group(0) @binding(0) var<uniform> u_params: GravitationalPotentialParams;

struct GravitationalPotentialParams {
    u_G: f32; // Gravitational constant
    u_M: f32; // Mass of the Moon
    u_c: f32; // Speed of light
};

@fragment
fn main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    let r = length(pos.xy); // Distance from the Moon's center
    let potential = sqrt(1.0 - (2.0 * u_params.u_G * u_params.u_M) / (r * u_params.u_c * u_params.u_c));

    // Map the gravitational potential to color
    let color = vec3<f32>(potential, 0.0, 1.0 - potential);
    return vec4<f32>(color, 1.0);
}
