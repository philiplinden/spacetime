use hifitime::{Duration, Epoch, TimeScale};
use krabmaga::{
    engine::{location::Real2D, schedule::Schedule, state::State},
    log,
    rand::{self, Rng},
};

use crate::model::time::CoordinateTime;

use super::{
    agents::IdealTimekeeper,
    time::{ClockBehavior as _, IdealClock},
};

pub struct WorldSpace {
    pub step: u64,
    /// The dimensions of the world space grid. Used to initialize or reset the
    /// world coordinate frame.
    dim: (f32, f32),
    /// The number of agents in the world. Used to initialize or reset the
    /// simulation with the desired number of agents.
    num_agents: u32,
    /// The duration of simulation time that advances with each step.
    pub delta_time: Duration,
    /// The coordinate frame's current time. This is the "true" time as observed
    /// by an external observer infinitely far away, i.e. the coordinate time.
    pub time: CoordinateTime,
}

impl WorldSpace {
    pub fn new(
        dim: (f32, f32),
        num_agents: u32,
        delta_time: Duration,
        start_epoch: Epoch,
        timescale: TimeScale,
    ) -> Self {
        Self {
            step: 0,
            dim,
            num_agents,
            delta_time,
            time: CoordinateTime::new(timescale, Some(start_epoch)),
        }
    }
}

impl Default for WorldSpace {
    fn default() -> Self {
        Self::new(
            (100., 100.),
            10,
            Duration::from_seconds(0.0),
            Epoch::from_unix_seconds(0.0),
            TimeScale::UTC,
        )
    }
}

impl State for WorldSpace {
    /// Agent creation and schedule set-up
    fn init(&mut self, schedule: &mut Schedule) {
        self.step = 0;

        let mut rng = rand::thread_rng();

        for i in 0..self.num_agents {
            // Generate random properties for the agent
            let r1: f32 = rng.gen();
            let r2: f32 = rng.gen();
            let r3: f32 = rng.gen();
            // PLACEHOLDER: Set the position of the agent, even if it's not used
            let loc = Real2D {
                x: self.dim.0 * r1,
                y: self.dim.1 * r2,
            };
            let agent = IdealTimekeeper {
                id: i,
                position: loc,
                velocity: Real2D::default(),
                clock: IdealClock::new(CoordinateTime::default()),
                mass: r3,
            };
            // Put the agent in your state
            schedule.schedule_repeating(Box::new(agent), 0., 0);
        }
    }

    fn update(&mut self, _step: u64) {
        self.step += 1;
        self.time += self.delta_time;
        log!(LogType::Info, format!("Coordinate time: {:?}", self.time));
    }

    fn reset(&mut self) {
        self.step = 0;
        self.time = self.time.reset();
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn as_state(&self) -> &dyn State {
        self
    }

    fn as_state_mut(&mut self) -> &mut dyn State {
        self
    }
}
