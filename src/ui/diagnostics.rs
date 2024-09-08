use bevy::prelude::*;
use avian2d::prelude::PhysicsDebugPlugin;

pub(super) fn plugin(app: &mut App) {
    #[cfg(feature = "dev")]
    {
        app.add_plugins((
            PhysicsDebugPlugin::default(),
            #[cfg(feature = "inspect")]
            bevy_inspector_egui::quick::WorldInspectorPlugin::default(),
        ));
        #[cfg(feature = "inspect")]
        {
            app.insert_resource(bevy_mod_picking::debug::DebugPickingMode::Normal);
        }
    }
}
