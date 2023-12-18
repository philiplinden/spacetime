use crate::{DISCRETIZATION, TOROIDAL};
use crate::model::clock::Clock;

use krabmaga::engine::{
    fields::{field::Field, field_2d::Field2D},
    schedule::Schedule,
    state::State
};
use std::any::Any;

pub struct Realm {
    pub step: u64,
    pub field: Field2D<Clock>,
    pub dim: (f32, f32),
    pub num_agents: u32,
}

impl Realm {
    pub fn new(dim: (f32, f32), num_agents: u32) -> Realm {
        Realm {
            step: 0,
            field: Field2D::new(dim.0, dim.1, DISCRETIZATION, TOROIDAL),
            dim,
            num_agents,
        }
    }
}

impl State for Realm {
    /// Put the code that should be executed for each state update here. The state is updated once for each
    /// schedule step.
    fn update(&mut self, _step: u64) {
        self.field.lazy_update();
    }

    /// Put the code that should be executed to reset simulation state
    fn reset(&mut self) {
        self.step = 0;
    }

    /// Put the code that should be executed to initialize simulation:
    /// Agent creation and schedule set-up
    fn init(&mut self, schedule: &mut Schedule) {
        self.step = 0;
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_state_mut(&mut self) -> &mut dyn State {
        self
    }

    fn as_state(&self) -> &dyn State {
        self
    }
    fn after_step(&mut self, _schedule: &mut Schedule) {
        self.step += 1
    }
}
