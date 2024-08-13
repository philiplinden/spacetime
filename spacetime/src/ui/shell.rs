use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use hifitime::TimeScale;
use crate::physics::time::CoordinateTime;

/// The main Ui that contains everything else. This should be just Egui sugar that triggers events handled elsewhere.
pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_shell);
}

fn update_shell(
    mut egui_ctx: EguiContexts,
    mut coordinate_time: ResMut<CoordinateTime>,
) {
    let ctx = egui_ctx.ctx_mut();
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                super::datetime::set_time_menu(ui, &mut coordinate_time);
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                egui::warn_if_debug_build(ui);
                ui.label(format!("Coordinate Time: {:?}", coordinate_time.as_ref()));
            });
        });
    });
}
