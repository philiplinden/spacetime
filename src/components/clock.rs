use bevy::prelude::*;
use hifitime::prelude::Epoch;

#[derive(Component, Debug)]
pub struct Clock {
    pub time: Epoch,
}

pub struct ClockPlugin;

impl Plugin for ClockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_clock);
    }
}

fn update_clock(mut query: Query<&mut Clock>) {
    for mut clock in query.iter_mut() {
        clock.time = Epoch::now().unwrap();
    }
}

pub struct TimeDilationSystem;

impl TimeDilationSystem {
    fn run(&mut self, commands: &mut Commands, time: Res<Time>, query: Query<&mut Clock>) {
        // Implement time dilation logic here
        // ...
    }
}

pub struct TimeResource {
    pub delta_seconds: f64,
}
