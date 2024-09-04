#[allow(dead_code)]

use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;

pub struct SatellitesPlugin;

impl Plugin for SatellitesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<Satellites>::new(&["ron"]));
        app.add_systems(Startup, load_satellites);
    }
}

#[derive(Asset, Debug, Deserialize, Reflect)]
pub struct Satellite {
    pub name: String,
    pub mass: f64,
    pub dimensions: [f64; 3],
    pub semi_major_axis: f64,
    pub eccentricity: f64,
    pub inclination: f64,
    pub orbital_period: f64,
    pub launch_date: String,
    pub power: Option<f64>,
    pub orbit_type: Option<String>,
    pub clock_type: Option<String>,
    pub clock_stability: Option<f64>,
    pub clock_frequency: Option<f64>,
    pub mission_end_date: Option<String>,
    pub time_dilation_correction: Option<f64>,
}

#[derive(Asset, Debug, Deserialize, Reflect)]
pub struct Satellites {
    pub satellites: Vec<Satellite>,
}

fn load_satellites(
    asset_server: Res<AssetServer>,
    satellites: Res<Assets<Satellites>>,
) {
    // Load the RON file
    let _handle: Handle<Satellites> = asset_server.load("config/satellites.ron");

    // After the RON file is loaded, we can access the satellite data
    for satellite_data in satellites.iter() {
        for satellite in &satellite_data.1.satellites {
            println!("Satellite: {:?}", satellite);
        }
    }
}
