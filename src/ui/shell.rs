use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

/// The main Ui that contains everything else. This should be just Egui sugar that triggers events handled elsewhere.
pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_shell);
}

fn update_shell(mut egui_ctx: EguiContexts, time: Res<Time>) {
    let ctx = egui_ctx.ctx_mut();
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                egui::warn_if_debug_build(ui);
                ui.label(format!(
                    "Elapsed Time: {:.2} seconds",
                    time.elapsed_seconds()
                ));
            });
        });
    });
}
