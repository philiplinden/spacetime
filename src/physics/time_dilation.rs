use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
    app.add_systems(Update, (update_time_dilation, update_ui));
}

#[derive(Component)]
struct TimeDilation {
    proper_time: f32,
    coordinate_time: f32,
    velocity: f32, // Fraction of the speed of light
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(TimeDilation {
        proper_time: 0.0,
        coordinate_time: 0.0,
        velocity: 0.1, // Example value for v/c
    });
}

fn update_time_dilation(mut query: Query<&mut TimeDilation>, time: Res<Time>) {
    for mut time_dilation in query.iter_mut() {
        time_dilation.coordinate_time += time.delta_seconds();
        time_dilation.proper_time +=
            time.delta_seconds() * (1.0 / (1.0 - time_dilation.velocity.powi(2)).sqrt());
    }
}

fn update_ui(
    mut egui_context: EguiContexts,
    query: Query<&TimeDilation>
) {
    let time_dilation = query.single();
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        ui.label(format!(
            "Coordinate Time: {:.2}\nProper Time: {:.2}",
            time_dilation.coordinate_time,
            time_dilation.proper_time
        ));
    });
}
