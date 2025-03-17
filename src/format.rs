#![allow(unused)]

use std::fmt::Display;
use bevy::prelude::*;
use uom::si::Quantity;

pub(crate) fn plugin(app: &mut App) {
    app.register_type::<UomQuantity>();
}

#[derive(Component, Debug, Reflect)]
pub struct UomQuantity {
    value: f32,
    unit: String,
}

impl UomQuantity {
    pub fn new<D, U, V>(quantity: &Quantity<D, U, V>) -> Self
    where
        D: uom::si::Dimension + ?Sized,
        U: uom::si::Units<V> + ?Sized + uom::si::Unit,
        V: uom::num::Num + uom::Conversion<V> + Into<f32> + Clone,
    {
        Self {
            value: quantity.value.clone().into(),
            unit: U::abbreviation().to_string(),
        }
    }
}

impl Display for UomQuantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.value, self.unit)
    }
}
