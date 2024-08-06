use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

pub(super) fn plugin(app: &mut App) {
    let debug_level: DebugPickingMode;
    #[cfg(not(feature = "dev"))]
    {
        debug_level = DebugPickingMode::Disabled;
    }
    #[cfg(feature = "dev")]
    {
        debug_level = DebugPickingMode::Normal;
    }
    app.add_plugins(DefaultPickingPlugins).insert_resource(debug_level);
}
