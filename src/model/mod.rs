pub mod time;
pub mod dynamics;
pub mod objects;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        time::plugin,
        dynamics::plugin,
        objects::plugin,
    ));
}
