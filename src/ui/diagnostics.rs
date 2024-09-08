use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    #[cfg(feature = "dev")]
    {
        app.add_plugins(());
        #[cfg(feature = "inspect")]
        {
            app.add_plugins((
                avian2d::prelude::PhysicsDebugPlugin::default(),
                bevy_inspector_egui::quick::WorldInspectorPlugin::default(),
            ));
            app.insert_resource(bevy_mod_picking::debug::DebugPickingMode::Normal);
        }
    }
}
