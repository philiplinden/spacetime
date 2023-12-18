use crate::model::spacetime::Realm;
use core::fmt;
use krabmaga::engine::{
    agent::Agent,
    fields::field_2d::{toroidal_transform, Location2D},
    location::Real2D,
    state::State,
};
use std::hash::{Hash, Hasher};

/// The most basic agent should implement Clone, Copy and Agent to be able to be inserted in a Schedule.
#[derive(Clone, Copy)]
pub struct Clock {
    pub id: u32,
    pub loc: Real2D,
    pub last_d: Real2D,
    pub dir_x: f32,
    pub dir_y: f32,
}

impl Agent for Clock {
    /// Put the code that should happen for each step, for each agent here.
    fn step(&mut self, state: &mut dyn State) {
        let state = state.as_any().downcast_ref::<Realm>().unwrap();

        let loc_x = toroidal_transform(self.loc.x + self.dir_x, state.field.width);
        let loc_y = toroidal_transform(self.loc.y + self.dir_y, state.field.height);
        self.loc = Real2D { x: loc_x, y: loc_y };

        state
            .field
            .set_object_location(*self, Real2D { x: loc_x, y: loc_y });
    }

    /// Put the code that decides if an agent should be removed or not
    /// for example in simulation where agents can die
    fn is_stopped(&mut self, _state: &mut dyn State) -> bool {
        false
    }
}

impl Hash for Clock {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl Location2D<Real2D> for Clock {
    fn get_location(self) -> Real2D {
        self.loc
    }

    fn set_location(&mut self, loc: Real2D) {
        self.loc = loc;
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Eq for Clock {}

impl PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        self.id == other.id
    }
}
