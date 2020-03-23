use crate::cb_simulation;
use cb_simulation::components;
use cb_simulation::CbSystemValues;

use specs::prelude::*;
pub struct IkSystem;

use crate::cb_menu;

use crate::cb_input;

impl<'a> System<'a> for IkSystem {
    type SystemData = (
        Read<'a, CbSystemValues>,
        WriteStorage<'a, components::ik_components::IkComponent>,
    );

    fn run(&mut self, (sys_values, mut ik_components): Self::SystemData) {
        let world_inputs = &sys_values.world_inputs;
    }
}
