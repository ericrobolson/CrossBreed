// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

// Note; do this using ECS

extern crate specs;
use specs::prelude::*;

use crate::cb_math;
use cb_math::cb_range::CbNormalizedRange;

use crate::cb_inverse_kinematics;
use cb_inverse_kinematics::IkRig;

use crate::cb_menu;

pub struct IkComponent {
    pub rig: IkRig,
}

impl IkComponent {
    pub fn new() -> Self {
        return Self { rig: IkRig::new() };
    }
}

init_component_implementations![IkComponent];
