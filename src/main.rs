use krabmaga::*;
use hifitime::{Duration, Epoch};

mod components;
mod model;
use crate::model::Realm;

fn main() {
    pretty_env_logger::init();
    let num_agents = 1;
    let start_epoch = Epoch::now().unwrap();
    let time_step = Duration::from_nanoseconds(1.0);

    let steps = 100;
    let reps = 1;
    let state = Realm::new(num_agents, start_epoch, time_step);

    simulate!(state, steps, reps);
}
