#[allow(dead_code)]

use bevy::prelude::*;
use serde::Deserialize;

pub struct CelestialsPlugin;

impl Plugin for CelestialsPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, load_celestials);
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

// fn load_celestials(asset_server: Res<AssetServer>, celestials: Res<Assets<Celestials>>) {
//     // Load the RON file
//     let _handle: Handle<Celestials> = asset_server.load("config/celestials.ron");

//     // After the RON file is loaded, we can access the celestial data
//     for celestial_data in celestials.iter() {
//         for celestial in &celestial_data.1.celestials {
//             println!("Celestial Object: {:?}", celestial);
//         }
//     }
// }
