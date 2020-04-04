// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

extern crate specs;
use specs::prelude::*;

use crate::cb_system;
use cb_system::Coordinate2d;

use crate::cb_math;
use cb_math::FInt;

use super::ComponentLinker;

init_components![
    PhysicsComponentsLinker,
    (VelocityComponent, TransformComponent)
];

pub struct VelocityComponent(Coordinate2d);

impl VelocityComponent {
    pub fn new() -> Self {
        return Self(Coordinate2d::zero());
    }
}

pub struct TransformComponent {
    pub world_position: Coordinate2d,
    pub rotation: Coordinate2d,
    pub scale: Coordinate2d,
}

impl TransformComponent {
    pub fn new() -> Self {
        let zero = FInt::from_num(0);

        return Self {
            world_position: Coordinate2d::zero(),
            rotation: Coordinate2d::zero(),
            scale: Coordinate2d::one(),
        };
    }
}
