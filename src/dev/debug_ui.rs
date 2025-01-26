use bevy::{
    dev_tools::ui_debug_overlay::{DebugUiPlugin, UiDebugOptions},
    input::common_conditions::input_just_pressed,
    prelude::*,
};

const TOGGLE_KEY: KeyCode = KeyCode::F4;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        DebugUiPlugin,
    )).add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
    );
}

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}
