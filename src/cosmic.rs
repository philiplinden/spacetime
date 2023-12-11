use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use uom::si::f32::{Length, Mass};
use uom::si::{
    length::{kilometer, meter},
    mass::kilogram,
};

pub struct InitWorldPlugin;

impl Plugin for InitWorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .insert_resource(AmbientLight {
                color: Color::default(),
                brightness: 0.75,
            })
            .add_systems(Startup, (spawn_earth, spawn_moon));
    }
}

#[derive(Component, Debug)]
struct CelestialBody {
    name: String,
    radius: Length,
    mass: Mass,
    model_path: String,
}

impl CelestialBody {
    fn new(name: String, radius: Length, mass: Mass, model_path: String) -> Self {
        CelestialBody {
            name,
            radius,
            mass,
            model_path,
        }
    }

    fn from_preset(name: &str) -> Self {
        let preset = match name {
            "Earth" => Ok(CelestialBody {
                name: String::from("Earth"),
                radius: Length::new::<kilometer>(6_378.0),
                mass: Mass::new::<kilogram>(5.9722E24),
                model_path: String::from("models/Earth.glb#Scene0"),
            }),
            "Luna" => Ok(CelestialBody {
                name: String::from("Luna"),
                radius: Length::new::<kilometer>(1_737.4),
                mass: Mass::new::<kilogram>(7.342E22),
                model_path: String::from("models/Moon.glb#Scene0"),
            }),
            // "Sun" => Ok(CelestialBody {
            //     name: String::from("Sun"),
            //     radius: Length::new::<kilometer>(696_342.0),
            //     mass: Mass::new::<kilogram>(1.98847E30),
            //     model_path: String::from("models/Sun.glb#Scene0"),
            // }),
            _ => Err(()),
        };
        preset.unwrap()
    }
}

#[derive(Bundle)]
struct CelestialBodyBundle {
    body: CelestialBody,
    model: SceneBundle,
    collider: Collider,
}

impl CelestialBodyBundle {
    fn new(name: &str, position: Vec3, asset_server: Res<AssetServer>) -> Self {
        let body = CelestialBody::from_preset(name);
        let radius = body.radius.get::<meter>();
        let asset = asset_server.load(&body.model_path);
        CelestialBodyBundle {
            body,
            model: SceneBundle {
                scene: asset,
                transform: Transform::from_xyz(position.x, position.y, position.z),
                ..Default::default()
            },
            collider: Collider::ball(radius),
        }
    }
}

fn spawn_earth(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(CelestialBodyBundle::new("Earth", Vec3::ZERO, asset_server));
}

fn spawn_moon(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(CelestialBodyBundle::new(
        "Luna",
        Vec3::new(1.0E3, 0.0, 0.0),
        asset_server,
    ));
}
