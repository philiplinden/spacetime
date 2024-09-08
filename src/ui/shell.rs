use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::physics::time::CoordinateTime;

/// The main Ui that contains everything else. This should be just Egui sugar that triggers events handled elsewhere.
pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_shell);
}

fn update_shell(
    mut egui_ctx: EguiContexts,
    coordinate_time: Res<CoordinateTime>,
) {
    let ctx = egui_ctx.ctx_mut();
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                egui::warn_if_debug_build(ui);
                ui.label(format!("Scale: {:?}", coordinate_time.scale));
                // ui.label(format!("Coordinate Time: {:}", coordinate_time.epoch()));
                ui.label(format!(
                    "{:} ({:.4} elapsed seconds)",
                    coordinate_time.epoch(),
                    coordinate_time.elapsed_seconds(),
                ))
            });
        });
    });
}
