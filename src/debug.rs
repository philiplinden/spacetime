use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::components::clock;

pub struct DebugToolsPlugin;

impl Plugin for DebugToolsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Update, print_clock_time);
    }
}

fn print_clock_time(query: Query<(Entity, &clock::Clock)>) {
    for (entity, clock) in query.iter() {
        info!("Entity {:?} has time {:?}", entity, clock.time.to_rfc3339())
    }
}
