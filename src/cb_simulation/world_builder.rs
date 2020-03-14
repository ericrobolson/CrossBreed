extern crate specs;
use specs::prelude::*;

use super::*;
use components;
use components::physics_components;

pub fn new() -> specs::World {
    let mut world = World::new();

    // Physics components
    {
        world.register::<physics_components::VelocityComponent>();
        world.register::<physics_components::TransformComponent>();
    }

    return world;
}
