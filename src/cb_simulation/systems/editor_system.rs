use crate::cb_simulation;
use cb_simulation::components;
use cb_simulation::CbSystemValues;

use specs::prelude::*;
pub struct EditorSystem;

impl<'a> System<'a> for EditorSystem {
    type SystemData = (
        Write<'a, CbSystemValues>,
        WriteStorage<'a, components::EditableComponent>,
        WriteStorage<'a, components::voxel_components::VoxelComponent>,
    );

    fn run(
        &mut self,
        (mut sys_values, mut editable_components, mut voxel_components): Self::SystemData,
    ) {
        // If no entity selected, wait for selection
        // TODO: add ability to create entities on the fly?

        for (editable, voxel) in (&mut editable_components, &mut voxel_components).join() {
            let mut databinding_changes = voxel.handle_events(&sys_values.events, sys_values.frame);

            sys_values
                .databinding_changes
                .append(&mut databinding_changes);

            // sync stuff
        }
    }
}
