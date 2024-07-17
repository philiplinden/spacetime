use bevy::prelude::*;

pub struct ScenarioPlugin;

impl Plugin for ScenarioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_scenario);
    }
}

fn init_scenario(
    mut commands: Commands,
    mut event_writer: EventWriter<ComputePredictionEvent>,
    physics: Res<PhysicsSettings>,
) {
    let earth = body::BodySetting {
        name: "Earth",
        position: Vec3::new(0.0, 0.0, 0.0) * SCALE,
        mu: G * 5.97e24 * SCALE,
        radius: 6378.0e3 * SCALE,
        material: StandardMaterial {
            base_color: Color::rgb(0.0, 0.6, 1.0),
            ..default()
        },
        ..default()
    };

    let moon = body::BodySetting {
        name: "Moon",
        position: earth.position + Vec3::new(384_400.0e3, 0.0, 0.0) * SCALE,
        mu: G * 0.073e24 * SCALE,
        radius: 1737.5e3 * SCALE,
        material: StandardMaterial {
            base_color: Color::rgb(0.6, 0.4, 0.1),
            ..default()
        },
        ..default()
    }
    .orbiting(&earth, Vec3::new(0.0, 0.5, -1.0));

    // This should be turned into a macro
    let mut earth_bundle = body::BodyBundle::new(earth);
    earth_bundle.prediction_bundle.draw.steps = Some(0);
    let earth = commands.spawn(earth_bundle).id();

    let mut moon_bundle = body::BodyBundle::new(moon);
    moon_bundle.prediction_bundle.draw.reference = Some(earth);
    commands.spawn(moon_bundle);

    commands.insert_resource(Followed(Some(earth)));

    event_writer.send(ComputePredictionEvent {
        steps: physics.steps_per_second() * 60 * 5,
    });
}
