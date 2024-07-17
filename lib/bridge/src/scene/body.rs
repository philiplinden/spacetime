use bevy::prelude::*;

use crate::gui::{
    label::Labelled,
    select::{CanFollow, CanSelect},
};
use spacetime_core::{
    kinematics::{Acceleration, Mass, Position, Velocity},
    Interpolated, SCALE,
    orbit_prediction::{PredictionBundle, PredictionDraw},
};

// Planetoids, stars, and other natural bodies are called CELESTIALS
// Artificial bodies are called SPACECRAFT or STATION

#[derive(Component, Clone)]
pub struct BodyMaterial {
    pub mesh: Mesh,
    pub material: StandardMaterial,
}

impl Default for BodyMaterial {
    fn default() -> Self {
        Self {
            mesh: shape::Cube { size: 10.0 }.into(),
            material: StandardMaterial::default(),
        }
    }
}

pub fn add_materials(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &BodyMaterial), Added<BodyMaterial>>,
) {
    for (entity, material) in &query {
        let mut cmds = commands.entity(entity);

        let BodyMaterial { mesh, material } = material.clone();
        if material.emissive != Color::BLACK {
            cmds.with_children(|child| {
                child.spawn(PointLightBundle {
                    point_light: PointLight {
                        color: material.emissive,
                        intensity: 5E4,
                        range: 2E3 * SCALE,
                        shadows_enabled: true,
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..default()
                });
            });
        }

        cmds.insert(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(material),
            ..default()
        });
    }
}

#[derive(Bundle, Default)]
pub struct ParticleBundle {
    pub interolated: Interpolated,
    pub acceleration: Acceleration,
    pub velocity: Velocity,
    pub position: Position,
    pub mass: Mass,
}

#[derive(Bundle, Default)]
pub struct BodyBundle {
    pub name: Name,
    pub labelled: Labelled,
    pub can_select: CanSelect,
    pub can_follow: CanFollow,
    pub body_material: BodyMaterial,
    pub particle_bundle: ParticleBundle,
    pub prediction_bundle: PredictionBundle,
}

#[derive(Default, Clone)]
pub struct BodySetting {
    pub name: &'static str,
    pub velocity: Vec3,
    pub position: Vec3,
    pub mu: f32,
    pub radius: f32,
    pub material: StandardMaterial,
}

impl BodySetting {
    pub fn orbiting(mut self, orbiting: &Self, axis: Vec3) -> Self {
        let distance = self.position - orbiting.position;

        self.velocity = distance.cross(axis).normalize()
            * ((self.mu + orbiting.mu) / distance.length()).sqrt()
            + orbiting.velocity;

        self
    }
}

impl BodyBundle {
    pub fn new(setting: BodySetting) -> Self {
        Self {
            name: Name::new(setting.name),
            labelled: Labelled {
                style: TextStyle {
                    font_size: 6.0 * (1000.0 * setting.radius).log10(),
                    color: Color::GRAY,
                    ..default()
                },
                offset: Vec2::splat(setting.radius) * 1.1,
            },
            can_select: CanSelect {
                radius: setting.radius,
            },
            can_follow: CanFollow {
                min_camera_distance: setting.radius * 3.0,
            },
            particle_bundle: ParticleBundle {
                mass: Mass(setting.mu),
                velocity: Velocity(setting.velocity),
                position: Position(setting.position),
                ..default()
            },
            prediction_bundle: PredictionBundle {
                draw: PredictionDraw {
                    color: setting.material.base_color,
                    ..default()
                },
                ..default()
            },
            body_material: BodyMaterial {
                mesh: shape::UVSphere {
                    radius: setting.radius,
                    ..default()
                }
                .into(),
                material: setting.material,
            },
        }
    }
}
