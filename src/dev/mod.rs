mod console;
mod debug_ui;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    console::plugin(app);
    debug_ui::plugin(app);
}
