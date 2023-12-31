use krabmaga::*;

mod model;
use crate::model::Realm;


fn main() {
    pretty_env_logger::init();

    let step = 100;
    let state = Realm::default();

    simulate!(state, step, 10);
}
