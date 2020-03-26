use crate::cb_simulation;
use cb_simulation::components;
use cb_simulation::CbSystemValues;

use specs::prelude::*;
pub struct EditorSystem;

use crate::cb_menu;
use cb_menu::EditorComponent;

impl<'a> System<'a> for EditorSystem {
    type SystemData = (
        Write<'a, CbSystemValues>,
        WriteStorage<'a, components::EditableComponent>,
        WriteStorage<'a, components::voxel_components::VoxelComponent>,
        WriteStorage<'a, components::audio_components::FmSynthComponent>,
    );

    fn run(
        &mut self,
        (mut sys_values, mut editable_components, mut voxel_components, mut fm_synth_components): Self::SystemData,
    ) {
        // If no entity selected, wait for selection
        // TODO: add ability to create entities on the fly?

        for (editable, voxel) in (&mut editable_components, &mut voxel_components).join() {
            if editable.is_editing() {
                let mut databinding_changes =
                    voxel.handle_events(&sys_values.events, sys_values.frame);

                sys_values
                    .databinding_changes
                    .append(&mut databinding_changes);
            }

            // sync stuff
        }

        for (editable, synth) in (&mut editable_components, &mut fm_synth_components).join() {
            if editable.is_editing() {
                let mut databinding_changes =
                    synth.handle_events(&sys_values.events, sys_values.frame);

                sys_values
                    .databinding_changes
                    .append(&mut databinding_changes);
            }

            // sync stuff
        }
    }
}
