use crate::cb_simulation;
use cb_simulation::components;
use cb_simulation::CbSystemValues;
use components::RangePresentableTestComponent;

use specs::prelude::*;
pub struct EditorSystem;

impl<'a> System<'a> for EditorSystem {
    type SystemData = (
        Read<'a, CbSystemValues>,
        WriteStorage<'a, RangePresentableTestComponent>,
    );

    fn run(&mut self, (sys_values, mut presentercomponents): Self::SystemData) {
        // If no entity selected, wait for selection
        // TODO: add ability to create entities on the fly?

        for presenter_component in (&mut presentercomponents).join() {
            presenter_component.range.value += 10; // Here to test game sim updating presenter
            presenter_component.sync();
        }
    }
}
