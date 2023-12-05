use crate::model::world::World;
use core::fmt;
use krabmaga::{
    engine::{
        agent::Agent,
        state::State,
    },
    log,
};
use std::hash::{Hash, Hasher};

/// The most basic agent should implement Clone, Copy and Agent to be able to be inserted in a Schedule.
#[derive(Clone, Copy)]
pub struct Body {
    pub id: u32,
    pub mass: f32,
}

impl Body {
    pub fn new(id: u32, mass: f32) -> Self {
        Body {
            id,
            mass,
        }
    }
}

impl Agent for Body {
    /// Put the code that should happen for each step, for each agent here.
    fn step(&mut self, state: &mut dyn State) {
        log!(LogType::Info, format!("Step agent {}", self.id));
        let state = state.as_any().downcast_ref::<World>().unwrap();
    }

    /// Put the code that decides if an agent should be removed or not
    /// for example in simulation where agents can die
    fn is_stopped(&mut self, _state: &mut dyn State) -> bool {
        false
    }
}

impl Hash for Body {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Eq for Body {}

impl PartialEq for Body {
    fn eq(&self, other: &Body) -> bool {
        self.id == other.id
    }
}
