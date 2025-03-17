use avian3d::prelude::*;
use bevy::{
    prelude::*,
    asset::AssetMetaCheck,
};

#[cfg(feature = "inspect")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                // Wasm builds will check for meta files (that don't exist) if this isn't set.
                // This causes errors and even panics on web build on itch.
                // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Window {
                    title: "s p a c e t i m e".to_string(),
                    canvas: Some("#bevy".to_string()),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: true,
                    ..default()
                }
                .into(),
                ..default()
            })
    );
    app.init_state::<AppState>();
    app.add_systems(OnEnter(AppState::Paused), pause);
    app.add_systems(OnExit(AppState::Paused), unpause);

    #[cfg(feature = "inspect")]
    app.add_plugins(WorldInspectorPlugin::new());
}

#[derive(States, Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum AppState {
    #[default]
    Running,
    Paused,
}

pub fn pause(mut physics_time: ResMut<Time<Physics>>, mut next_state: ResMut<NextState<AppState>>) {
    physics_time.as_mut().pause();
    debug!("pausing physics time");
    next_state.set(AppState::Paused);
}

pub fn unpause(
    mut physics_time: ResMut<Time<Physics>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    physics_time.as_mut().unpause();
    debug!("unpausing physics time");
    next_state.set(AppState::Running);
}
