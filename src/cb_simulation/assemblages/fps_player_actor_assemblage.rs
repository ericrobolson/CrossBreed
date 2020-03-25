use crate::cb_simulation::components;
use components::{
    actor_components, gfx_components, ik_components, physics_components, voxel_components,
};
use gfx_components::CameraComponent;
use physics_components::TransformComponent;
use voxel_components::VoxelComponent;

extern crate specs;
use specs::prelude::*;

pub fn new(world: &mut World) {
    world
        .create_entity()
        .with(TransformComponent::new())
        .with(actor_components::ActorComponent::new())
        .with(ik_components::IkComponent::new())
        .with(components::EditableComponent::new(false))
        .build();
}
