use crate::cb_simulation::components;
use components::audio_components;

extern crate specs;
use specs::prelude::*;

pub fn new(world: &mut World) {
    world
        .create_entity()
        .with(audio_components::FmSynthComponent::new())
        .with(components::EditableComponent::new(true))
        .build();
}
