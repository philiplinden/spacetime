pub mod debug;
pub mod hud;
pub mod orbit_prediction;
pub mod select;
pub mod label;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub struct BaseUiPlugin;

impl Plugin for BaseUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .insert_resource(ClearColor(Color::BLACK))
            .insert_resource(AmbientLight {
                color: Color::NONE,
                brightness: 0.0,
            })
            .add_systems(PostStartup, setup_egui);
    }
}

fn setup_egui(mut ctxs: EguiContexts) {
    ctxs.ctx_mut().set_visuals(egui::Visuals {
        window_fill: egui::Color32::from_rgba_premultiplied(27, 27, 27, 225),
        window_stroke: egui::Stroke::NONE,
        ..egui::Visuals::dark()
    });
}

pub fn format_duration(duration: std::time::Duration, precision: usize) -> String {
    humantime::format_duration(duration)
        .to_string()
        .split_inclusive(' ')
        .take(precision)
        .collect::<String>()
}