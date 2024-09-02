use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<SpawnPlanetoid>();
    app.add_systems(Update, spawn_planetoid);
}

#[derive(Component)]
pub struct Planetoid;

#[derive(Bundle)]
struct CircleBody {
    physics_body: RigidBody,
    collider_shape: Collider,
    graphics_shape: ShapeBundle,
    fill: Fill,
}

impl CircleBody {
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
        CircleBody {
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


#[derive(Event)]
pub struct SpawnPlanetoid {
    pub position: Vec2,
    pub velocity: Vec2,
    pub mass: f32,
    pub radius: f32,
}

fn spawn_planetoid(mut commands: Commands, mut events: EventReader<SpawnPlanetoid>) {
    let text_style = TextStyle {
        font_size: 18.0,
        ..default()
    };
    for e in events.read(){
        commands
            .spawn((
                Name::new("Hello"),
                Planetoid,
                CircleBody::from_radius(e.radius)
            ))
            .with_children(|subcommands| {
                subcommands.spawn((
                    Text2dBundle {
                        transform: Transform::from_translation(Vec3::new(10.0, -10.0, 10.0)),
                        text: Text::from_section("", text_style.clone())
                            .with_justify(JustifyText::Left),
                        text_anchor: bevy::sprite::Anchor::TopLeft,
                        ..default()
                    },
                ));
            });
    }
}
