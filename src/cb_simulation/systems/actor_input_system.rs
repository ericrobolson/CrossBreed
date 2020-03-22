use crate::cb_simulation;
use cb_simulation::components;
use cb_simulation::CbSystemValues;

use specs::prelude::*;
pub struct ActorInputSystem;

use crate::cb_menu;

use crate::cb_input;

impl<'a> System<'a> for ActorInputSystem {
    type SystemData = (
        Read<'a, CbSystemValues>,
        WriteStorage<'a, components::actor_components::ActorComponent>,
    );

    fn run(&mut self, (sys_values, mut actor_components): Self::SystemData) {
        let world_inputs = &sys_values.world_inputs;

        for actor_component in (&mut actor_components).join() {
            let actor_inputs: std::vec::Vec<&cb_input::cb_input::CbGameInput> = world_inputs
                .iter()
                .filter(|i| actor_component.player_id == i.player_id)
                .collect();

            // Add inputs to actors
            for input in actor_inputs.iter() {
                actor_component.inputs.push(**input);
            }
        }
    }
}
