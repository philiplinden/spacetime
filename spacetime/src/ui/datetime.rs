use bevy::prelude::*;
use bevy_egui::egui::{self, Widget};
use egui_extras::DatePickerButton;
use hifitime::prelude::*;
use lofitime::{HifiEpoch, LofiDateTime};

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
            let hifi_epoch = HifiEpoch(coordinate_time.epoch());
            let chrono_datetime: chrono::NaiveDateTime = hifi_epoch.into();
            let mut selected_date = chrono_datetime.date();
            if DatePickerButton::new(&mut selected_date).ui(ui).changed() {
                // Update the Time resource with the selected date
                let new_time = selected_date.and_time(chrono::NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap());
                let new_datetime = new_time.and_utc();
                let lofi_datetime = LofiDateTime(new_datetime);
                coordinate_time.start_epoch = Some(lofi_datetime.into());
            }
        });
    });
}
