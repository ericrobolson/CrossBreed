use crate::cb_simulation;
use cb_simulation::components;
use cb_simulation::CbSystemValues;

use specs::prelude::*;
pub struct EditorSystem;

impl<'a> System<'a> for EditorSystem {
    type SystemData = (
        Read<'a, CbSystemValues>,
        WriteStorage<'a, components::EditableComponent>,
        WriteStorage<'a, components::voxel_components::VoxelComponent>,
    );

    fn run(
        &mut self,
        (sys_values, mut editable_components, mut voxel_components): Self::SystemData,
    ) {
        // If no entity selected, wait for selection
        // TODO: add ability to create entities on the fly?

        for (editable, voxel) in (&mut editable_components, &mut voxel_components).join() {

            // sync stuff
        }
    }
}
