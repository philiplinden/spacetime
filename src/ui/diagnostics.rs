use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    #[cfg(feature = "dev")]
    {
        use bevy_inspector_egui;
        app.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::default());
    }
}
