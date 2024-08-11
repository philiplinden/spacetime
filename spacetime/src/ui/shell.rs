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
                set_time_menu(ui, &mut coordinate_time);
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                egui::warn_if_debug_build(ui);
                ui.label(format!("Coordinate Time: {:?}", coordinate_time.as_ref()));
            });
        });
    });
}


fn set_time_menu(ui: &mut egui::Ui, coordinate_time: &mut ResMut<CoordinateTime>) {
    ui.menu_button("Coordinate Time...", |ui| {
        ui.menu_button("Scale...", |ui| {
            ui.vertical(|ui| {
                ui.radio_value(&mut coordinate_time.scale, TimeScale::UTC, "UTC");
                ui.radio_value(&mut coordinate_time.scale, TimeScale::TAI, "TAI");
                ui.radio_value(&mut coordinate_time.scale, TimeScale::BDT, "BDT");
            });
        });
        ui.menu_button("Epoch...", |ui| {
            // ui.add(egui_extras::DatePickerButton::new(&mut coordinate_time.start_epoch));
        });
    });
}
