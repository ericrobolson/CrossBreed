extern crate specs;
use specs::prelude::*;

use super::*;

use components;
use components::{
    actor_components, gfx_components, ik_components, physics_components, voxel_components,
    EditableComponent,
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

    // IK components
    {
        world.register::<ik_components::IkComponent>();
    }

    // Actor components
    {
        world.register::<actor_components::ActorComponent>();
    }

    // Misc components
    {
        world.register::<EditableComponent>();
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

        assemblages::fps_player_actor_assemblage::new(&mut world);
    }

    return world;
}
