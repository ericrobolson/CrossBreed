// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

extern crate specs;
use specs::prelude::*;

use crate::cb_system;
use cb_system::Coordinate;

use super::ComponentLinker;

init_components![
    PhysicsComponentsLinker,
    (VelocityComponent, TransformComponent)
];

pub struct VelocityComponent(Coordinate);

impl VelocityComponent {
    pub fn new() -> Self {
        return Self(Coordinate::new(0, 0, 0));
    }
}

pub struct TransformComponent {
    pub world_position: Coordinate,
    pub rotation: Coordinate,
    pub scale: Coordinate,
}

impl TransformComponent {
    pub fn new() -> Self {
        return Self {
            world_position: Coordinate::new(0, 0, 0),
            rotation: Coordinate::new(0, 0, 0),
            scale: Coordinate::new(
                Coordinate::full_unit(),
                Coordinate::full_unit(),
                Coordinate::full_unit(),
            ),
        };
    }
}
