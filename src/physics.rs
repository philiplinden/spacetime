use avian3d::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default().set(PhysicsInterpolationPlugin::interpolate_all()));
    app.insert_resource(Gravity(Vec3::ZERO)); // we compute gravity ourselves
}
