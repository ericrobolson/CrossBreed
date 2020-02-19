extern crate specs;
use specs::prelude::*;

use crate::cb_system;
use cb_system::{Coordinate, GameUnit};

pub struct VelocityComponent(Coordinate);

impl VelocityComponent {
    pub fn new() -> Self {
        return Self(Coordinate::new(0, 0, 0));
    }
}

pub struct TransformComponent {}

impl TransformComponent {
    pub fn new() -> Self {
        return Self {};
    }
}

init_component_implementations![VelocityComponent, TransformComponent];
