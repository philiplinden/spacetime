use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::prelude::*;

use crate::gui::select::{Clickable, CanFollow, Followed};


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

        cmds.insert((
            meshes.add(mesh),
            materials.add(material),
            VisibilityBundle::default(),
        ));
    }
}

#[derive(Bundle, Default)]
pub struct ParticleBundle {
    pub rigidbody: RigidBody,
    pub collider: Collider,
    pub velocity: Velocity,
    pub friction: Friction,
    pub transform: TransformBundle,
    pub mass: ColliderMassProperties,
    pub read_mass: ReadMassProperties,
}

#[derive(Bundle, Default)]
pub struct BodyBundle {
    pub can_select: Clickable,
    pub can_follow: CanFollow,
    pub body_material: BodyMaterial,
    pub particle_bundle: ParticleBundle,
}

pub fn spawn_bodies(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 50000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            rotation: Quat::from_rotation_x(std::f32::consts::PI / 2.5),
            ..default()
        },
        ..default()
    });

    let main_mu = 5e7;

    let main = commands
        .spawn(BodyBundle {
            can_select: Clickable { radius: 100.0 },
            can_follow: CanFollow {
                min_camera_distance: 200.0,
            },
            body_material: BodyMaterial {
                mesh: shape::UVSphere {
                    radius: 100.0,
                    ..default()
                }
                .into(),
                material: StandardMaterial {
                    base_color: Color::rgb(0.6, 0.4, 0.2),
                    ..default()
                },
            },
            particle_bundle: ParticleBundle {
                rigidbody: RigidBody::Dynamic,
                collider: Collider::ball(100.0),
                mass: ColliderMassProperties::Mass(main_mu),
                ..default()
            },
        })
        .id();

    commands.insert_resource(Followed(Some(main)));

    // SPAWN DEBRIS
    let mut rng = thread_rng();
    let mut gen = || rng.gen_range(-8.0..=8.0) * 2.0;

    let radius = 1.0;
    let minor_mu = 25.0;

    for _ in 0..2000 {
        let distance = Vec3::new(315.0 + gen(), gen(), gen());
        let velocity =
            distance.cross(Vec3::Z).normalize() * ((main_mu + minor_mu) / distance.length()).sqrt();

        commands.spawn(BodyBundle {
            can_select: Clickable { radius },
            can_follow: CanFollow {
                min_camera_distance: 5.0,
            },
            body_material: BodyMaterial {
                mesh: shape::UVSphere {
                    radius,
                    ..default()
                }
                .into(),
                material: StandardMaterial {
                    base_color: Color::rgb(0.5, 0.5, 0.5),
                    ..default()
                },
            },
            particle_bundle: ParticleBundle {
                rigidbody: RigidBody::Dynamic,
                collider: Collider::ball(radius),
                velocity: Velocity::linear(velocity),
                friction: Friction::coefficient(0.8),
                transform: TransformBundle::from(Transform::from_translation(distance)),
                mass: ColliderMassProperties::Mass(minor_mu),
                ..default()
            },
        });
    }
}