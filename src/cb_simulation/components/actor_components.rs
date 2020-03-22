// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

extern crate specs;
use specs::prelude::*;

use crate::cb_simulation;
use cb_simulation::CbWorldInputs;

pub struct ActorComponent {
    pub player_id: usize,
    pub inputs: CbWorldInputs,
}

impl ActorComponent {
    pub fn new() -> Self {
        return Self {
            player_id: 1, //NOTE: this is a bug, need to figure out some way to increment player ids?
            inputs: vec![],
        };
    }
}

init_component_implementations![ActorComponent];
