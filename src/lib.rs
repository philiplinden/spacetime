mod app;
mod physics;
mod format;
mod grid;

use bevy::prelude::*;
pub struct SpacetimePlugin;

impl Plugin for SpacetimePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            app::plugin,
            physics::plugin,
            format::plugin,
            grid::plugin,
        ));
    }
}
