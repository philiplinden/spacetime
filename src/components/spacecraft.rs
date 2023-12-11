use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct InitEntitiesPlugin;

impl Plugin for InitEntitiesPlugin {
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
    commands
        .spawn(SatelliteBundle {
            model: SceneBundle {
                scene: model,
                transform: Transform::from_xyz(0.0, 1.0E7, 0.0),
                ..Default::default()
            },
            collider: Collider::cuboid(2.0, 1.0, 1.0)
        });
}
