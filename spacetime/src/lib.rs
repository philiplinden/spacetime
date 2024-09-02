mod physics;
mod ui;
pub mod components;
pub mod palette;

use bevy::prelude::*;
pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {

        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "spacetime".to_string(),
                        canvas: Some("#bevy".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
        );

        // Add custom plugins.
        app.add_plugins((
            ui::UserInterfacePlugins,
            physics::plugin,
            components::plugin,
        ));
        app.add_systems(PostStartup, initialize);

        // Initialize the app state enum
        app.init_state::<AppState>();

        // Change the background color.
        app.insert_resource(ClearColor(palette::BEVY_GRAY));
    }
}


#[derive(Resource, Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    Splash,
    Loading,
    Paused,
    #[default]
    Running,
}

fn initialize(mut events: EventWriter<components::bodies::SpawnPlanetoid>) {
    // spawn center body
    events.send(components::bodies::SpawnPlanetoid {
        position: Vec2::new(0.0, 0.0),
        velocity: Vec2::new(0.0, 0.0),
        mass: 100_000.0,
        radius: 10.0,
    });
}
