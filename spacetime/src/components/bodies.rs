use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

#[derive(Component)]
pub struct Planetoid;

#[derive(Bundle)]
pub struct PlanetoidBundle {
    physics_body: RigidBody,
    collider_shape: Collider,
    graphics_shape: ShapeBundle,
    fill: Fill,
}

impl Default for PlanetoidBundle {
    fn default() -> Self {
        let default_radius = 10.0;
        Self::from_radius(default_radius)
    }
}

impl PlanetoidBundle {
    pub fn from_radius(radius: f32) -> Self {
        let points = [
            Vec2::new(1.0, -1.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(-1.0, 1.0),
            Vec2::new(-1.0, -1.0),
        ].map(|x| x * radius);
        let shape = shapes::RoundedPolygon {
            points: points.into_iter().collect(),
            radius: radius,
            ..default()
        };
        PlanetoidBundle {
            physics_body: RigidBody::Dynamic,
            collider_shape: Collider::circle(radius),
            graphics_shape: ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                ..default()
            },
            fill: Fill::color(bevy::color::palettes::basic::WHITE),
        }
    }
}
