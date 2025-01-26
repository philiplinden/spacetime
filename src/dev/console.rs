use bevy::{
    prelude::*,
    log::{self, LogPlugin},
};
use bevy_console::{self, make_layer, AddConsoleCommand, ConsoleCommand, ConsoleConfiguration, ConsoleOpen};
use clap::Parser;

const OPEN_BY_DEFAULT: bool = false;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        bevy_console::ConsolePlugin,
        LogPlugin {
            level: log::Level::INFO,
            filter: "info,capture_bevy_logs=info".to_owned(),
            custom_layer: make_layer,
        }))
        .insert_resource(ConsoleOpen {
            open: OPEN_BY_DEFAULT,
        })
        .insert_resource(ConsoleConfiguration {
            top_pos: 0.0,
            left_pos: 5.0,
            height: 500.0,
            width: 1280.0,
            show_title_bar: false,
            ..Default::default()
        });

    // Add commands plugins
    app.add_plugins((
        SpawnCommandsPlugin,
    ));
}

struct SpawnCommandsPlugin;

impl Plugin for SpawnCommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_console_command::<SpawnCommand, _>(spawn_command)
            .add_console_command::<DespawnCommand, _>(despawn_command);
    }
}

/// Spawns an entity of the specified type
#[derive(Parser, ConsoleCommand)]
#[command(name = "spawn")]
struct SpawnCommand {
    /// Type of entity to spawn
    entity_type: String,
    /// Optional name for the entity
    #[arg(short, long)]
    name: Option<String>,
}

fn spawn_command(mut log: ConsoleCommand<SpawnCommand>, mut commands: Commands) {
    if let Some(Ok(SpawnCommand { entity_type, name })) = log.take() {
        match entity_type.as_str() {
            "something" => {
                let entity_name = name.clone().unwrap_or_else(|| entity_type.clone());
                let entity = commands.spawn(
                    Name::new(entity_name),
                ).id();
                log.reply_ok(format!("Spawned new {} with entity id: {:?}", entity_type, entity));
            }
            _ => {
                log.reply_failed(format!("Unknown entity type: {}", entity_type));
            }
        }
    }
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "despawn")]
struct DespawnCommand {
    /// Entity to remove
    entity_id: u32,
}

fn despawn_command(mut log: ConsoleCommand<DespawnCommand>, mut commands: Commands) {
    if let Some(Ok(DespawnCommand { entity_id })) = log.take() {
        commands
            .entity(Entity::from_raw(entity_id))
            .despawn_recursive();
        log.ok();
    }
}
