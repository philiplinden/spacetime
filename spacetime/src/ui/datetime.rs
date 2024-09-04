use bevy::prelude::*;
use bevy_egui::egui::{self, Widget};
use egui_extras::DatePickerButton;
use hifitime::prelude::*;
use lofitime::{HifiEpoch, LofiDateTime};

use crate::physics::time::CoordinateTime;

pub fn set_time_menu(mut commands: Commands, ui: &mut egui::Ui, coordinate_time: &mut ResMut<CoordinateTime>) {
    let mut scale: TimeScale = coordinate_time.scale;
    let mut start_epoch: Option<Epoch> = Some(coordinate_time.start_epoch.unwrap_or_default());
    ui.menu_button("Coordinate Time...", |ui| {
        ui.menu_button("Scale...", |ui| {
            ui.vertical(|ui| {
                ui.radio_value(&mut scale, TimeScale::UTC, "UTC");
                ui.radio_value(&mut scale, TimeScale::TAI, "TAI");
                ui.radio_value(&mut scale, TimeScale::BDT, "BDT");
            });
        });
        ui.menu_button("Epoch...", |ui| {
            let hifi_epoch = HifiEpoch(start_epoch.unwrap());
            let chrono_datetime: chrono::NaiveDateTime = hifi_epoch.into();
            let mut selected_date = chrono_datetime.date();
            if DatePickerButton::new(&mut selected_date).ui(ui).changed() {
                // Update the Time resource with the selected date
                let new_time = selected_date
                    .and_time(chrono::NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap());
                let new_datetime = new_time.and_utc();
                let lofi_datetime = LofiDateTime(new_datetime);
                start_epoch = Some(lofi_datetime.into());
            }
        });
    });
    // Updating the fields directly doesn't seem to update the epoch, so let's
    // try this instead. We want to carry over the existing elapsed time.
    commands.insert_resource::<CoordinateTime>(CoordinateTime {
        scale,
        elapsed: coordinate_time.elapsed.clone(),
        start_epoch,
    });
}
