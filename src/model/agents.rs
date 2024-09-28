use krabmaga::engine::{agent::Agent, location::Real2D, state::State};

use super::{
    state::WorldSpace,
    time::{ClockBehavior, IdealClock},
};

/// An agent that has an ideal clock that ticks at a constant rate.
/// TODO: Make a generic Timekeeper and implement an Ideal trait for it
#[derive(Debug, Clone, Copy)]
pub struct IdealTimekeeper {
    pub id: u32,
    pub position: Real2D,
    pub velocity: Real2D,
    pub clock: IdealClock,
    pub mass: f32,
}

impl IdealTimekeeper {
    pub fn new(id: u32, position: Real2D, velocity: Real2D, clock: IdealClock, mass: f32) -> Self {
        Self {
            id,
            position,
            velocity,
            clock,
            mass,
        }
    }
}

impl Agent for IdealTimekeeper {
    fn step(&mut self, state: &mut dyn State) {
        let state = state
            .as_any()
            .downcast_ref::<WorldSpace>()
            .expect("State should be of type WorldSpace");

        self.clock.tick(state.delta_time);
    }
}

// TODO: Make a RealTimekeeper to simulate clocks with noise
