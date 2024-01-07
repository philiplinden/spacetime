mod debug;
pub use debug::DebugUiPlugin;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_pancam::{PanCam, PanCamPlugin};

pub struct BaseUiPlugin;

impl Plugin for BaseUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EguiPlugin, PanCamPlugin::default()))
            .add_systems(Startup, setup_camera)
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

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera3dBundle::default(), PanCam::default()));
}
