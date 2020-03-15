// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.
extern crate specs;
use specs::prelude::*;

use crate::cb_system;
use cb_system::{Coordinate, GameUnit};

use crate::cb_voxels;

pub struct VoxelComponent {
    pub chunk_manager: cb_voxels::CbChunkManager,
}

impl VoxelComponent {
    pub fn new() -> Self {
        return Self {
            chunk_manager: cb_voxels::CbChunkManager::new(),
        };
    }
}

init_component_implementations![VoxelComponent];
