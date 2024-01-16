use bevy::prelude::*;

use crate::{SCALED_LENGTH, SCALED_MASS};
use crate::components::body::{BodyBundle, BodySetting};
use crate::gui::{
    orbit_prediction::ComputePredictionEvent,
    select::{Followed, Selected},
};
use crate::physics::{PhysicsSettings, constants::G};

pub struct ScenarioPlugin;

impl Plugin for ScenarioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, scene_cislunar_real_scale);
    }
}

fn scene_cislunar_real_scale(
    mut commands: Commands,
    mut event_writer: EventWriter<ComputePredictionEvent>,
    physics: Res<PhysicsSettings>,
) {
    let earth = BodySetting {
        name: "Earth",
        position: Vec3::new(0.0, 0.0, 0.0) * SCALED_LENGTH,
        mu: G * 5.97e24 * SCALED_MASS,
        radius: 6378.0e3 * SCALED_LENGTH,
        material: StandardMaterial {
            base_color: Color::rgb(0.0, 0.6, 1.0),
            ..default()
        },
        ..default()
    };

    let moon = BodySetting {
        name: "Moon",
        position: earth.position + Vec3::new(384_400.0e3, 0.0, 0.0) * SCALED_LENGTH,
        mu: G * 0.073e24 * SCALED_MASS,
        radius: 1737.5e3 * SCALED_LENGTH,
        material: StandardMaterial {
            base_color: Color::rgb(0.6, 0.4, 0.1),
            ..default()
        },
        ..default()
    }
    .orbiting(&earth, Vec3::new(0.0, 0.5, -1.0));

    // This should be turned into a macro
    let mut earth_bundle = BodyBundle::new(earth);
    earth_bundle.prediction_bundle.draw.steps = Some(0);
    let earth = commands.spawn(earth_bundle).id();

    let mut moon_bundle = BodyBundle::new(moon);
    moon_bundle.prediction_bundle.draw.reference = Some(earth);
    commands.spawn(moon_bundle);

    commands.insert_resource(Followed(Some(earth)));

    event_writer.send(ComputePredictionEvent {
        steps: physics.steps_per_second() * 60 * 5,
    });
}

fn scene_pocket_solar_system(
    mut commands: Commands,
    mut event_writer: EventWriter<ComputePredictionEvent>,
    physics: Res<PhysicsSettings>,
) {
    let star_color = Color::rgb(1.0, 1.0, 0.9);
    let star = BodySetting {
        name: "Star",
        velocity: Vec3::new(0.0, 0.0, 0.0),
        mu: 5E3,
        radius: 8.0,
        material: StandardMaterial {
            base_color: star_color,
            emissive: star_color * 2.0,
            ..default()
        },
        ..default()
    };

    let planet = BodySetting {
        name: "Planet",
        position: Vec3::new(0.0, 60.0, 0.0),
        mu: 100.0,
        radius: 2.0,
        material: StandardMaterial {
            base_color: Color::rgb(0.0, 0.6, 1.0),
            ..default()
        },
        ..default()
    }
    .orbiting(&star, Vec3::Z);

    let moon = BodySetting {
        name: "Moon",
        position: planet.position + Vec3::new(4.5, 0.0, 0.0),
        mu: 1.0,
        radius: 0.6,
        material: StandardMaterial {
            base_color: Color::rgb(0.6, 0.4, 0.1),
            ..default()
        },
        ..default()
    }
    .orbiting(&planet, Vec3::new(0.0, 0.5, -1.0));

    let comet = BodySetting {
        name: "Comet",
        velocity: Vec3::new(2.8, 0.15, 0.4),
        position: Vec3::new(-200.0, 138.0, -18.0),
        mu: 0.000,
        radius: 0.1,
        material: StandardMaterial {
            base_color: Color::rgb(0.3, 0.3, 0.3),
            ..default()
        },
    };

    let mut star_bundle = BodyBundle::new(star);
    star_bundle.prediction_bundle.draw.steps = Some(0);
    let star = commands.spawn((star_bundle, Selected)).id();

    let mut planet_bundle = BodyBundle::new(planet);
    planet_bundle.prediction_bundle.draw.reference = Some(star);
    let planet = commands.spawn(planet_bundle).id();

    let mut moon_bundle = BodyBundle::new(moon);
    moon_bundle.prediction_bundle.draw.reference = Some(planet);
    commands.spawn(moon_bundle);

    let mut comet_bundle = BodyBundle::new(comet);
    comet_bundle.prediction_bundle.draw.reference = Some(star);
    commands.spawn(comet_bundle);

    commands.insert_resource(Followed(Some(star)));

    event_writer.send(ComputePredictionEvent {
        steps: physics.steps_per_second() * 60 * 5,
    });
}
