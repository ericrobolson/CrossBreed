extern crate specs;
use specs::prelude::*;

use super::*;
use cb_patterns;
use components;
use components::{
    gfx_components, physics_components, voxel_components, EditableComponent,
    RangePresentableTestComponent,
};

pub fn new(mode: CbSimulationModes) -> specs::World {
    let mut world = World::new();

    // Physics components
    {
        world.register::<physics_components::VelocityComponent>();
        world.register::<physics_components::TransformComponent>();
    }

    // Voxel components
    {
        world.register::<voxel_components::VoxelComponent>();
    }

    // GFX components
    {
        world.register::<gfx_components::CameraComponent>();
    }

    // Misc components
    {
        world.register::<EditableComponent>();
        world.register::<RangePresentableTestComponent>();
    }

    // Resources
    {
        world.insert(CbSystemValues::new());
    }

    // Setup entities
    {
        if mode == CbSimulationModes::VoxelEditor {
            // run all voxel editor assemblages
            assemblages::voxel_editor_assemblages::new(&mut world);
        }
    }

    return world;
}
