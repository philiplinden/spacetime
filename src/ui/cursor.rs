use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(DefaultPickingPlugins);
}
