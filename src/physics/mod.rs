pub mod time;

use avian2d::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        PhysicsPlugins::default().with_length_unit(1.0),
        time::plugin,
    ));

    // we will handle gravity ourselves
    app.insert_resource(Gravity(Vec2::ZERO.into()));
}
