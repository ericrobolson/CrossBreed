use crate::cb_simulation::components;
use components::{gfx_components, physics_components, voxel_components};
use gfx_components::CameraComponent;
use physics_components::TransformComponent;
use voxel_components::VoxelComponent;

extern crate specs;
use specs::prelude::*;

pub fn new(world: &mut World) {
    world
        .create_entity()
        .with(VoxelComponent::new())
        .with(TransformComponent::new())
        .with(CameraComponent::new())
        .build();
}
