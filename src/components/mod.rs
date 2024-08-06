use bevy::prelude::*;
use bevy_prototype_lyon::plugin::ShapePlugin;

pub mod bodies;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(ShapePlugin);
}
