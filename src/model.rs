use std::any::Any;

use hifitime::{Duration, Epoch};
use krabmaga::engine::{
    agent::Agent,
    schedule::Schedule,
    state::State,
};
use krabmaga::log;
use uuid::Uuid;

use crate::components::Clock;

// ---------------------------------------------------------------------------------------------------------------------

/// Realms are the connective tissue that binds the state of the simulation and
/// the "world" or field where agents exist. Realm implements the krABMaaga
/// State trait to trigger updates to the Schedule and mutate the world State.
pub struct Realm {
    pub step: u64,
    pub num_agents: u32,
    start_epoch: Epoch,
    pub epoch: Epoch,
    pub time_step: Duration,
}

impl Default for Realm {
    fn default() -> Self {
        Realm {
            step: 0,
            num_agents: 10,
            start_epoch: Epoch::from_unix_seconds(0.),
            epoch: Epoch::from_unix_seconds(0.),
            time_step: Duration::from_nanoseconds(1.0),
        }
    }
}

impl Realm {
    pub fn new(num_agents: u32, start_epoch: Epoch, time_step: Duration) -> Realm {
        Realm {
            step: 0,
            num_agents,
            start_epoch,
            epoch: start_epoch,
            time_step,
        }
    }
}

impl State for Realm {
    /// The state is updated once for each schedule step.
    fn update(&mut self, _step: u64) {
        log!(LogType::Info, format!("True epoch {:?}", self.epoch));
    }

    /// Reset simulation state (without creating a new one)
    fn reset(&mut self) {
        self.step = 0;
        self.epoch = self.start_epoch;
    }

    /// Agent creation and schedule set-up
    fn init(&mut self, schedule: &mut Schedule) {
        for _new_agent in 0..self.num_agents {
            let node = Node::new(self.epoch);
            schedule.schedule_repeating(Box::new(node), 0., 0);
        }
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
        self.step += 1;
        self.epoch = self.epoch + self.time_step;
    }
}

// ---------------------------------------------------------------------------------------------------------------------

/// Nodes participate in the simulation. They exist within a Realm, and the Realm's schedule manages when they update.
/// The Agent implementation here manages what happens when an update is triggered on a given Node.
#[derive(Clone)]
struct Node {
    uuid: Uuid,
    clock: Clock,
}

impl Node {
    fn new(epoch: Epoch) -> Self {
        Node {
            uuid: Uuid::new_v4(),
            clock: Clock::new(epoch),
        }
    }
}

impl Agent for Node {
    fn step(&mut self, state: &mut dyn State) {
        let realm: &Realm = state.as_any().downcast_ref().unwrap();
        self.clock.tick(realm.time_step);
        log!(LogType::Info, format!("{:}: {:?}", self.uuid, self.clock.epoch))
    }
}
