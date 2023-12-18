use std::any::Any;

use super::player::Player;
use crate::{DISCRETIZATION, TOROIDAL, TWOPI, SAVE_OUTPUT, SCALE};

use krabmaga::{
    addplot,
    engine::{
        fields::{field::Field, field_2d::Field2D},
        schedule::Schedule,
        state::State,
    },
    log, plot,
    rand::{self, Rng},
    PlotData, DATA,
};

/// Expand the state definition according to your model, for example by having a grid struct field to
/// store the agents' locations.
pub struct Arena {
    pub step: u64,
    pub field: Field2D<Player>,
    pub dim: (f32, f32),
    pub num_agents: u32,
}

impl Arena {
    pub fn new(dim: (f32, f32), num_agents: u32) -> Arena {
        Arena {
            step: 0,
            field: Field2D::new(dim.0, dim.1, DISCRETIZATION, TOROIDAL),
            dim,
            num_agents,
        }
    }
}

impl State for Arena {
    /// Put the code that should be executed for each state update here. The state is updated once for each
    /// schedule step.
    fn update(&mut self, _step: u64) {
        self.field.lazy_update();
        log!(LogType::Info, format!("step {}", _step));
    }

    /// Put the code that should be executed to reset simulation state
    fn reset(&mut self) {
        self.step = 0;
        self.field = Field2D::new(self.dim.0, self.dim.1, DISCRETIZATION, TOROIDAL);
    }

    /// Put the code that should be executed to initialize simulation:
    /// Agent creation and schedule set-up
    fn init(&mut self, schedule: &mut Schedule) {
        self.step = 0;

        let mut rng = rand::thread_rng();

        // Spawn all of the players that are not "it"
        for i in 0..self.num_agents {
            let r1: f32 = rng.gen();
            let r2: f32 = rng.gen();
            let r3: f32 = rng.gen();
            let r4: f32 = rng.gen();

            let x = self.dim.0 * r1;
            let y: f32 = self.dim.1 * r2;
            let heading_rad = TWOPI as f32 * r3;
            let speed = SCALE as f32 * (1. + r4);
            let mut agent = Player::new(x, y, heading_rad, speed);

            if i == 0 {
                // designate one agent to be the seeker
                agent.tagged();
            } else {
                agent.notit();
            }
            schedule.schedule_repeating(Box::new(agent), 0., 0);
        }
        addplot!(
            String::from("State"),
            String::from("t"),
            String::from("state"),
            SAVE_OUTPUT
        );
        addplot!(
            String::from("Map"),
            String::from("x"),
            String::from("y"),
            SAVE_OUTPUT
        );
        addplot!(
            String::from("Detections"),
            String::from("t"),
            String::from("n"),
            SAVE_OUTPUT
        );
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
    fn after_step(&mut self, schedule: &mut Schedule) {
        let players = schedule.get_all_events();
        for p in players {
            if let Some(player) = p.downcast_ref::<Player>() {
                plot!(
                    String::from("State"),
                    player.short_id(),
                    schedule.step as f64, 
                    player.state as u64 as f64
                );
                plot!(
                    String::from("Map"),
                    player.short_id(),
                    player.loc.x as f64, 
                    player.loc.y as f64
                );
                plot!(
                    String::from("Detections"),
                    player.short_id(),
                    schedule.step as f64, 
                    player.detection_signal.magnitude() as f64
                );
            }
        }

        self.step += 1
    }
}
