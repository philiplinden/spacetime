use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::model::time::{Clock, ClockCamera};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, display_clock_window);
}

fn display_clock_window(
    mut egui_ctx: EguiContexts,
    clock_query: Query<&Clock, With<ClockCamera>>,
) {
    let ctx = egui_ctx.ctx_mut();

    egui::Window::new("Clock Information").show(ctx, |ui| {
        if let Ok(clock) = clock_query.get_single() {
            ui.label("Clock Name: Camera Clock");
            ui.label(format!("Proper Time: {:.2}", clock.proper_time));
            ui.label(format!("Coordinate Time: {:.2}", clock.coordinate_time));
        } else {
            ui.label("No clock found");
        }
    });
}
