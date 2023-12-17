use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct InitParticipantsPlugin;

impl Plugin for InitParticipantsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_satellite);
    }
}

#[derive(Bundle)]
struct SatelliteBundle {
    model: SceneBundle,
    collider: Collider,
}

fn spawn_satellite(mut commands: Commands, asset_server: Res<AssetServer>) {
    let model = asset_server.load("models/Satellite.glb#Scene0");
    commands.spawn(SatelliteBundle {
        model: SceneBundle {
            scene: model,
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(0.1)),
            ..Default::default()
        },
        collider: Collider::cuboid(15.0, 20.0, 75.0),
    });
}
