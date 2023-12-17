use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use uom::si::f32::{Length, Mass};
use uom::si::{
    length::{kilometer, meter},
    mass::kilogram,
};

use crate::visuals::{camera::Followed, ui::Selected};

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
            "Sun" => Ok(CelestialBody {
                name: String::from("Sun"),
                radius: Length::new::<kilometer>(696_342.0),
                mass: Mass::new::<kilogram>(1.98847E30),
                model_path: String::from("models/Sun.glb#Scene0"),
            }),
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
    mass_props: ColliderMassProperties,
}

impl CelestialBodyBundle {
    fn new(name: &str, position: Vec3, asset_server: Res<AssetServer>) -> Self {
        let body = CelestialBody::from_preset(name);
        let radius_m = body.radius.get::<meter>();
        let mass_kg = body.mass.get::<kilogram>();
        info!(
            "Spawning {:?}: Radius {:?} m, Mass {:?} kg",
            name, radius_m, mass_kg
        );
        let asset = asset_server.load(&body.model_path);
        CelestialBodyBundle {
            body,
            model: SceneBundle {
                scene: asset,
                transform: Transform::from_xyz(position.x, position.y, position.z)
                    .with_scale(Vec3::splat(radius_m)),
                ..Default::default()
            },
            collider: Collider::ball(radius_m),
            mass_props: ColliderMassProperties::Mass(mass_kg),
        }
    }
}

fn spawn_earth(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(CelestialBodyBundle::new("Earth", Vec3::ZERO, asset_server));
}

fn spawn_moon(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(CelestialBodyBundle::new(
        "Luna",
        Vec3::new(384_400.0E3, 0.0, 0.0),
        asset_server,
    ));
}

fn spawn_sun(mut commands: Commands, asset_server: Res<AssetServer>) {
    let range = 149_597_871_000.0;
    let sun = CelestialBodyBundle::new("Sun", Vec3::new(range, 0.0, 0.0), asset_server);
    let radius = sun.body.radius.get::<meter>();
    let star = commands
        .spawn((
            sun,
            PointLight {
                radius,
                color: Color::rgb(1.0, 1.0, 0.9) * 2.0,
                intensity: range,
                range: range * 10.0,
                shadows_enabled: true,
                ..default()
            },
            Selected,
        ))
        .id();
    commands.insert_resource(Followed(Some(star)));
}

pub struct InitCelestialsPlugin;

impl Plugin for InitCelestialsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::GRAY))
            .insert_resource(AmbientLight {
                color: Color::default(),
                brightness: 0.75,
            })
            .add_systems(Startup, (spawn_sun, spawn_earth, spawn_moon));
    }
}
