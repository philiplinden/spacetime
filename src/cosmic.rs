use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub const EARTH_RADIUS_M: f32 = 6.3781;//E6;
pub const ORIGIN: Vec3 = Vec3::ZERO;

pub struct InitWorldPlugin;

impl Plugin for InitWorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::DARK_GRAY))
            .insert_resource(AmbientLight {
                color: Color::default(),
                brightness: 0.75,
            })
            .add_systems(Startup, spawn_earth);
    }
}

#[derive(Bundle)]
struct EarthBundle {
    model: SceneBundle,
    collider: Collider,
}

fn spawn_earth(mut commands: Commands, asset_server: Res<AssetServer>) {
    /* Create the Earth. */
    let gltf = asset_server.load("models/Earth.glb#Scene0");
    commands
        .spawn(EarthBundle {
            model: SceneBundle {
                scene: gltf,
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            },
            collider: Collider::ball(EARTH_RADIUS_M),
        });
}
