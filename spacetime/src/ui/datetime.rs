use bevy::prelude::*;
use bevy_egui::egui::{self, Widget};
use egui_extras::DatePickerButton;
use lofitime::HifiDateTime;
use hifitime::{Epoch, TimeScale};

use crate::physics::time::CoordinateTime;

pub fn set_time_menu(ui: &mut egui::Ui, coordinate_time: &mut ResMut<CoordinateTime>) {
    ui.menu_button("Coordinate Time...", |ui| {
        ui.menu_button("Scale...", |ui| {
            ui.vertical(|ui| {
                ui.radio_value(&mut coordinate_time.scale, TimeScale::UTC, "UTC");
                ui.radio_value(&mut coordinate_time.scale, TimeScale::TAI, "TAI");
                ui.radio_value(&mut coordinate_time.scale, TimeScale::BDT, "BDT");
            });
        });
        ui.menu_button("Epoch...", |ui| {
            let mut selected_date = (coordinate_time.epoch() as hifitime::Epoch).to_lofi_naive().date();
            if DatePickerButton::new(&mut selected_date).ui(ui).changed() {
                // Update the Time resource with the selected date
                let new_time = selected_date.and_time(chrono::NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap());
                coordinate_time = CoordinateTime {
                    scale: coordinate_time.scale,
                    start_epoch: Some(new_time.and_utc().to_hifi_epoch()),
                    ..default()
                };
            }
        });
    });
}
