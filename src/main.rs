use krabmaga::*;

use crate::model::sim::Realm;

mod model;

pub static TOROIDAL: bool = true;
pub static DISCRETIZATION: f32 = 1.0;

fn main() {
    let step = 100;
    let state = Realm::default();

    simulate!(state, step, 10);
}
