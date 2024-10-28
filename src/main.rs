mod constants;
mod simulation;
mod ui;

fn main() {
    App::new().add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "spacetime".to_string(),
                ..default()
            }),
            ..default()
        }),
        ui::InterfacePlugin,
        simulation::time::TimePlugin,
    ))
}

#[derive(States, Clone, Eq, PartialEq, Debug, Default, Hash)]
pub enum AppState {
    #[default]
    Menu,
    Loading,
    Playing,
    Editing,
}

#[derive(States, Clone, Eq, PartialEq, Debug, Default, Hash)]
pub enum WorldState {
    #[default]
    Paused,
    Running,
}
