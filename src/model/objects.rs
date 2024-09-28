use bevy::prelude::*;

use super::time::Clock;

pub(super) fn plugin(app: &mut App) {
    app
        .add_systems(Startup, setup_clock);
}

fn setup_clock(mut commands: Commands) {
    commands.spawn((
        Name::new("World Clock"),
        Clock::default(),
        TransformBundle::default(),
    ));
}
