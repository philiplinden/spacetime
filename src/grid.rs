use bevy::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.init_resource::<GridConfig>();
    app.add_systems(Update, draw_grid);
}

/// Configuration for the grid
#[derive(Resource)]
pub struct GridConfig {
    pub size: UVec3,        // Dimensions of the grid (cubes per axis)
    pub cube_size: f32,     // Size of each cube
    pub position: Vec3,     // Position of the grid center
    pub rotation: Quat,     // Rotation of the entire grid
    pub color: Color,       // Color of the grid lines
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            size: UVec3::new(20, 20, 1),
            cube_size: 10.0,
            position: Vec3::ZERO, // Center of the grid
            rotation: Quat::IDENTITY,
            color: Color::srgba(0.0, 1.0, 0.0, 0.5), // Green color
        }
    }
}

pub fn draw_grid(mut gizmos: Gizmos, grid_config: Res<GridConfig>) {
    let transform = Transform::from_translation(grid_config.position)
        .with_rotation(grid_config.rotation);

    // Calculate grid center offset
    let grid_extent = Vec3::new(
        grid_config.size.x as f32 * grid_config.cube_size,
        grid_config.size.y as f32 * grid_config.cube_size,
        grid_config.size.z as f32 * grid_config.cube_size
    );
    let center_offset = -grid_extent / 2.0;

    for x in 0..grid_config.size.x {
        for y in 0..grid_config.size.y {
            for z in 0..grid_config.size.z {
                let local_pos = Vec3::new(
                    x as f32 * grid_config.cube_size,
                    y as f32 * grid_config.cube_size,
                    z as f32 * grid_config.cube_size
                ) + center_offset;

                let position = transform.transform_point(local_pos);
                draw_wireframe_cube(
                    &mut gizmos,
                    position,
                    grid_config.cube_size,
                    transform.rotation,
                    grid_config.color
                );
            }
        }
    }
}

/// Draw a single wireframe cube
fn draw_wireframe_cube(
    gizmos: &mut Gizmos,
    position: Vec3,
    size: f32,
    rotation: Quat,
    color: Color
) {
    // Calculate the half-extents vectors in the rotated coordinate system
    let half_size = size / 2.0;
    let right = rotation * Vec3::X * half_size;
    let up = rotation * Vec3::Y * half_size;
    let forward = rotation * Vec3::Z * half_size;

    // Define the corners
    let corners = [
        position - right - up - forward, // 0: bottom back left
        position + right - up - forward, // 1: bottom back right
        position + right - up + forward, // 2: bottom front right
        position - right - up + forward, // 3: bottom front left
        position - right + up - forward, // 4: top back left
        position + right + up - forward, // 5: top back right
        position + right + up + forward, // 6: top front right
        position - right + up + forward, // 7: top front left
    ];

    // Bottom face
    gizmos.line(corners[0], corners[1], color);
    gizmos.line(corners[1], corners[2], color);
    gizmos.line(corners[2], corners[3], color);
    gizmos.line(corners[3], corners[0], color);

    // Top face
    gizmos.line(corners[4], corners[5], color);
    gizmos.line(corners[5], corners[6], color);
    gizmos.line(corners[6], corners[7], color);
    gizmos.line(corners[7], corners[4], color);

    // Vertical edges
    gizmos.line(corners[0], corners[4], color);
    gizmos.line(corners[1], corners[5], color);
    gizmos.line(corners[2], corners[6], color);
    gizmos.line(corners[3], corners[7], color);
}
