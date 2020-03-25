use crate::cb_simulation;
use cb_simulation::components;
use cb_simulation::CbSystemValues;

use specs::prelude::*;
pub struct FmAudioSystem;

use crate::cb_menu;
use cb_menu::EditorComponent;

impl<'a> System<'a> for FmAudioSystem {
    type SystemData = (Write<'a, CbSystemValues>,);

    fn run(&mut self, (mut sys_values): Self::SystemData) {}
}
