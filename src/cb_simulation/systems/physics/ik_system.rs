use crate::cb_simulation;
use cb_simulation::components;
use cb_simulation::CbSystemValues;

use crate::cb_inverse_kinematics;
use cb_inverse_kinematics::{fabrik, CbMatrix, IkRig};

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

        for ik_component in (&mut ik_components).join() {
            // Target the mouse coordinates
            let mouse_x = sys_values.editor_x;
            let mouse_y = sys_values.editor_y;

            let target = Some(CbMatrix::new(mouse_x as f32, mouse_y as f32));
            ik_component.rig.target = target;

            fabrik(&mut ik_component.rig);
        }
    }
}
