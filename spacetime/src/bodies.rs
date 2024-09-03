use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;

pub struct CelestialsPlugin;

impl Plugin for CelestialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<Celestials>::new(&["ron"]));
        app.add_systems(Startup, load_celestials);
    }
}

#[derive(Asset, Debug, Deserialize, Reflect)]
pub struct Celestial {
    pub name: String,
    pub mass: f64,
    pub radius: f64,
    pub position: [f64; 3],
    pub velocity: [f64; 3],
    pub gravitational_parameter: Option<f64>,
    pub schwarzschild_radius: Option<f64>,
    pub surface_gravity: Option<f64>,
    pub escape_velocity: Option<f64>,
    pub orbital_period: Option<f64>,
    pub axial_tilt: Option<f64>,
    pub rotation_period: Option<f64>,
    pub rotation_speed: Option<f64>,
}

#[derive(Asset, Debug, Deserialize, Reflect)]
pub struct Celestials {
    pub celestials: Vec<Celestial>,
}

fn load_celestials(asset_server: Res<AssetServer>, celestials: Res<Assets<Celestials>>) {
    // Load the RON file
    let handle: Handle<Celestials> = asset_server.load("config/celestials.ron");

    // After the RON file is loaded, we can access the celestial data
    for celestial_data in celestials.iter() {
        for celestial in &celestial_data.1.celestials {
            println!("Celestial Object: {:?}", celestial);
        }
    }
}

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
    let handle: Handle<Satellites> = asset_server.load("config/satellites.ron");

    // After the RON file is loaded, we can access the satellite data
    for satellite_data in satellites.iter() {
        for satellite in &satellite_data.1.satellites {
            println!("Satellite: {:?}", satellite);
        }
    }
}
