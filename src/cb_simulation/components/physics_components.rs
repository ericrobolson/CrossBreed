extern crate specs;
use specs::prelude::*;

use crate::cb_system;
use cb_system::Coordinate;

pub struct VelocityComponent(Coordinate);

impl VelocityComponent {
    pub fn new() -> Self {
        return Self(Coordinate::new(0, 0, 0));
    }
}

pub struct TransformComponent {
    pub position: Coordinate,
    pub transform: Coordinate,
}

impl TransformComponent {
    pub fn new() -> Self {
        return Self {
            position: Coordinate::new(0, 0, 0),
            transform: Coordinate::new(0, 0, 0),
        };
    }
}

init_component_implementations![VelocityComponent, TransformComponent];
