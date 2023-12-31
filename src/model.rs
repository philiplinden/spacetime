use krabmaga::engine::{
    schedule::{Schedule, ScheduleOptions},
    state::State, agent::Agent,
};
use std::any::Any;

/// Realms are the connective tissue that binds the state of the simulation and
/// the "world" or field where agents exist. Realm implements the krABMaaga
/// State trait to trigger updates to the Schedule and mutate the world State.
pub struct Realm {
    pub step: u64,
    pub num_agents: u32,
}

impl Default for Realm {
    fn default() -> Self {
        Realm {
            step: 0,
            num_agents: 10,
        }
    }
}

impl Realm {
    pub fn new(num_agents: u32) -> Realm {
        Realm {
            step: 0,
            num_agents,
        }
    }
}

impl State for Realm {
    /// The state is updated once for each schedule step.
    fn update(&mut self, _step: u64) {

    }

    /// Reset simulation state (without creating a new one)
    fn reset(&mut self) {
        self.step = 0;
    }

    /// Agent creation and schedule set-up
    fn init(&mut self, _schedule: &mut Schedule) {
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

/// Nodes participate in the simulation. They exist within a Realm, and the Realm's schedule manages when they update.
/// The Agent implementation here manages what happens when an update is triggered on a given Node.
#[derive(Clone, Copy)]
struct Node;

impl Agent for Node {
    fn is_stopped(&mut self, _state: &mut dyn State) -> bool {
        todo!();
    }

    fn before_step(
            &mut self,
            _state: &mut dyn State,
        ) -> Option<Vec<(Box<dyn Agent>, ScheduleOptions)>> {
        todo!();
    }

    fn step(&mut self, _state: &mut dyn State) {
        todo!();
    }

    fn after_step(
            &mut self,
            _state: &mut dyn State,
        ) -> Option<Vec<(Box<dyn Agent>, ScheduleOptions)>> {
        todo!();
    }
}