use crate::components::bodies::PlanetoidBundle;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_event::<SpawnPlanetoid>();
    app.add_systems(Update, spawn_planetoid);
}

#[derive(Event)]
pub struct SpawnPlanetoid {
    pub position: Vec2,
    pub velocity: Vec2,
    pub mass: f32,
    pub radius: f32,
}

fn spawn_planetoid(mut commands: Commands, mut events: EventReader<SpawnPlanetoid>) {
    for e in events.read(){
        commands.spawn(PlanetoidBundle::from_radius(e.radius));
    }
}
