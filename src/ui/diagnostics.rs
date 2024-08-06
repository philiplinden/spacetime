use bevy::prelude::*;
use iyes_perf_ui::prelude::*;
use avian2d::prelude::PhysicsDebugPlugin;

pub(super) fn plugin(app: &mut App) {
    #[cfg(feature = "dev")]
    {
        app.add_plugins((
            bevy_inspector_egui::quick::WorldInspectorPlugin::default(),
            PhysicsDebugPlugin::default(),
            // bevy::diagnostic::FrameTimeDiagnosticsPlugin,
            // bevy::diagnostic::EntityCountDiagnosticsPlugin,
            // bevy::diagnostic::SystemInformationDiagnosticsPlugin,
            // PerfUiPlugin,
        ));
        // app.add_systems(Startup, init);
    }
}

fn init(mut commands: Commands) {
    commands.spawn((
        PerfUiRoot::default(),
        PerfUiEntryFPS {
            #[cfg(debug_assertions)]
            label: "FPS [RUNNING IN DEBUG]".into(),
            ..Default::default()
        },
        PerfUiEntryFPSWorst::default(),
        PerfUiEntryFrameTime::default(),
        PerfUiEntryFrameTimeWorst::default(),
        PerfUiEntryEntityCount {
            digits: 7,
            threshold_highlight: None,
            color_gradient: ColorGradient::new_preset_gyr(50000.0, 200000.0, 500000.0).unwrap(),
            ..Default::default()
        },
        PerfUiEntryCpuUsage::default(),
        PerfUiEntryMemUsage::default(),
    ));
}
